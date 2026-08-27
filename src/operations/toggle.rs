use anyhow::{anyhow, Result};
use std::path::Path;

use crate::find::find_schemes;
use crate::operations::apply::apply;
use crate::operations::current::get_current_scheme;

/// Expand the given patterns into an ordered, deduplicated list of concrete
/// scheme names. Schemes matched by a single glob pattern are sorted
/// alphabetically, while the order of the patterns themselves is preserved.
fn resolve_schemes(patterns: &[&str], base_dir: &Path, config_dir: &Path) -> Result<Vec<String>> {
    let mut schemes: Vec<String> = Vec::new();
    for pattern in patterns {
        let found = find_schemes(pattern, base_dir, config_dir)?;
        let mut names: Vec<String> = found
            .iter()
            .filter_map(|path| path.file_stem())
            .filter_map(|stem| stem.to_str())
            .map(String::from)
            .collect();
        names.sort();
        for name in names {
            if !schemes.contains(&name) {
                schemes.push(name);
            }
        }
    }
    Ok(schemes)
}

/// Toggle subcommand
///
/// Cycles through the resolved schemes, applying the one that follows the
/// currently applied scheme (wrapping around). If no scheme has been applied
/// yet, or the current one isn't part of the list, the first scheme is applied.
///
/// * `patterns` - Schemes (or glob patterns) to cycle through
/// * `base_dir` - flavours base data dir
/// * `config_dir` - flavours config dir
/// * `config_path` - flavours configuration file
/// * `light_mode` - Skip running heavier hooks
/// * `verbose` - Should we be verbose?
pub fn toggle(
    patterns: Vec<&str>,
    base_dir: &Path,
    config_dir: &Path,
    config_path: &Path,
    light_mode: bool,
    verbose: bool,
) -> Result<()> {
    let schemes = resolve_schemes(&patterns, base_dir, config_dir)?;

    if schemes.is_empty() {
        return Err(anyhow!(
            "None of the given patterns matched an installed scheme. Check the names, or run 'flavours update all' first."
        ));
    }

    // Figure out which scheme is currently applied (if any)
    let current = get_current_scheme(base_dir).ok();

    // Pick the scheme that follows the current one, wrapping around
    let next = match current
        .as_deref()
        .and_then(|current| schemes.iter().position(|s| s == current))
    {
        Some(index) => &schemes[(index + 1) % schemes.len()],
        None => &schemes[0],
    };

    if verbose {
        match &current {
            Some(current) => println!("Toggling from {} to {}", current, next),
            None => println!("No current scheme found, applying {}", next),
        }
    }

    apply(
        vec![next.as_str()],
        base_dir,
        config_dir,
        config_path,
        light_mode,
        false,
        verbose,
    )
}
