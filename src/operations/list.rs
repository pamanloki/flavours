use anyhow::{anyhow, Result};
use std::path::Path;

use crate::find::find_schemes;
use crate::variant;

/// List subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours' base data dir
/// * `config_dir` - flavours' config dir
/// * `verbose` - Should we be verbose? (unused)
/// * `lines` - Should we print each scheme on its own line?
/// * `json` - Should we output a JSON array?
/// * `long` - When paired with `json`, emit enriched objects instead of slugs
pub fn list(
    patterns: Vec<&str>,
    base_dir: &Path,
    config_dir: &Path,
    _verbose: bool,
    lines: bool,
    json: bool,
    long: bool,
) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find_schemes(pattern, base_dir, config_dir)?;

        for found_scheme in found_schemes {
            schemes.push(String::from(
                found_scheme
                    .file_stem()
                    .ok_or_else(|| anyhow!("Couldn't get scheme name"))?
                    .to_str()
                    .ok_or_else(|| anyhow!("Couldn't convert name"))?,
            ));
        }
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };

    if json {
        if long {
            let entries: Vec<_> = schemes
                .iter()
                .map(|slug| {
                    serde_json::json!({
                        "slug": slug,
                        "family": variant::family(slug),
                        "mode": variant::variant(slug).as_str(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string(&entries)?);
        } else {
            println!("{}", serde_json::to_string(&schemes)?);
        }
        return Ok(());
    }

    for scheme in &schemes {
        // Print scheme
        print!("{}", scheme);
        if lines {
            // Print newline
            println!();
        } else {
            // Print space
            print!(" ");
        }
    }
    // If we separated by spaces, print an ending newline
    if !lines {
        println!();
    }

    Ok(())
}
