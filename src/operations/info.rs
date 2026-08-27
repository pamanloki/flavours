use anyhow::{anyhow, Context, Result};
use base16_color_scheme::{
    scheme::{RgbColor, RgbColorFormatter},
    template::color_field::{Format, Hex},
    Scheme,
};
use calm_io::stdoutln;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::colors_enabled;
use crate::find::find_schemes;

fn true_color(hex_color: &str, background: bool) -> Result<String> {
    let rgb = hex::decode(hex_color)?;

    let code = if background { 48 } else { 38 };

    Ok(format!("\x1b[{};2;{};{};{}m", code, rgb[0], rgb[1], rgb[2]))
}

pub fn print_color(color: &str) -> Result<()> {
    const RESETCOLOR: &str = "\x1b[0m";
    match stdoutln!(
        "{} #{} {}  {}#{}{}",
        true_color(color, true)?,
        color,
        RESETCOLOR,
        true_color(color, false)?,
        color,
        RESETCOLOR
    ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

pub fn print_color_rgb(color: RgbColor) -> Result<()> {
    use std::fmt::{self, Display, Formatter};

    struct TrueColor(RgbColor, bool);

    impl Display for TrueColor {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let RgbColor([r, g, b]) = self.0;
            let code = if self.1 { 48 } else { 38 };
            write!(f, "\x1b[{code};2;{r};{g};{b}m")
        }
    }

    const RESETCOLOR: &str = "\x1b[0m";

    let true_color_fg = TrueColor(color, true);
    let true_color_bg = TrueColor(color, false);

    let color = RgbColorFormatter {
        color,
        format: Format::Hex(Hex::Rgb),
    };
    match stdoutln!("{true_color_fg} #{color} {RESETCOLOR}  {true_color_bg}#{color}{RESETCOLOR}",) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

/// Format a color as a `#rrggbb` string.
pub fn hex_color(color: RgbColor) -> String {
    format!(
        "#{}",
        RgbColorFormatter {
            color,
            format: Format::Hex(Hex::Rgb)
        }
    )
}

/// Prints a plain (uncolored) hex line, ignoring broken pipes.
fn print_hex(color: RgbColor) -> Result<()> {
    match stdoutln!("{}", hex_color(color)) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

/// Collect the schemes matching every given pattern (sorted, deduplicated).
fn collect_schemes(patterns: Vec<&str>, base_dir: &Path, config_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find_schemes(pattern, base_dir, config_dir)?;
        schemes.extend_from_slice(&found_schemes);
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };
    Ok(schemes)
}

/// Read a scheme from a path, using the file name as its slug.
fn read_scheme(scheme_file: &Path) -> Result<Scheme> {
    let scheme_slug = scheme_file
        .file_stem()
        .ok_or_else(|| anyhow!("Couldn't get scheme name."))?
        .to_str()
        .ok_or_else(|| anyhow!("Couldn't convert scheme file name."))?;
    let scheme_contents = read_to_string(scheme_file)
        .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

    let mut scheme: Scheme = serde_yaml::from_str(&scheme_contents)?;
    scheme.slug = scheme_slug.to_string();
    Ok(scheme)
}

/// Emit every matching scheme as a JSON array.
fn info_json(patterns: Vec<&str>, base_dir: &Path, config_dir: &Path) -> Result<()> {
    let schemes = collect_schemes(patterns, base_dir, config_dir)?;

    let output: Vec<serde_json::Value> = schemes
        .iter()
        .map(|scheme_file| {
            let scheme = read_scheme(scheme_file)?;
            let colors: Vec<String> = scheme.colors.values().map(|&c| hex_color(c)).collect();
            Ok(serde_json::json!({
                "scheme": scheme.scheme,
                "slug": scheme.slug,
                "author": scheme.author,
                "path": scheme_file.to_string_lossy(),
                "colors": colors,
            }))
        })
        .collect::<Result<_>>()?;

    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

/// Info subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours base data dir
/// * `config_dir` - flavours config dir
/// * `raw` - Should we print raw (uncolored) hex codes?
/// * `json` - Should we output structured JSON instead?
pub fn info(
    patterns: Vec<&str>,
    base_dir: &Path,
    config_dir: &Path,
    raw: bool,
    json: bool,
) -> Result<()> {
    if json {
        return info_json(patterns, base_dir, config_dir);
    }

    // Fall back to plain hex output when colors are disabled (NO_COLOR / not a tty)
    let plain = raw || !colors_enabled();

    let schemes = collect_schemes(patterns, base_dir, config_dir)?;

    let mut first = true;
    for scheme_file in schemes {
        if first {
            first = false;
        } else {
            match stdoutln!() {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            }?;
        }

        let scheme = read_scheme(&scheme_file)?;

        match stdoutln!(
            "{} ({}) @ {}",
            scheme.scheme,
            scheme.slug,
            scheme_file.to_string_lossy()
        ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        match stdoutln!("by {}", scheme.author) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        for (_, &color) in scheme.colors.iter() {
            if plain {
                print_hex(color)?;
            } else {
                print_color_rgb(color)?;
            }
        }
    }

    Ok(())
}
