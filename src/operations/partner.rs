use anyhow::{anyhow, Result};
use std::path::Path;

use crate::find::find_schemes;
use crate::operations::current::get_current_scheme;
use crate::variant;

/// Collect the slugs of every installed scheme.
fn all_scheme_slugs(base_dir: &Path, config_dir: &Path) -> Result<Vec<String>> {
    let found = find_schemes("*", base_dir, config_dir)?;
    let mut slugs: Vec<String> = found
        .iter()
        .filter_map(|path| path.file_stem())
        .filter_map(|stem| stem.to_str())
        .map(String::from)
        .collect();
    slugs.sort();
    slugs.dedup();
    Ok(slugs)
}

/// Resolve the partner scheme for `slug` (or return an error).
pub fn resolve_partner(slug: &str, base_dir: &Path, config_dir: &Path) -> Result<String> {
    let all = all_scheme_slugs(base_dir, config_dir)?;
    variant::partner(slug, &all)
        .ok_or_else(|| anyhow!("No partner scheme found for '{}' (family '{}', looking for {} variant)",
            slug,
            variant::family(slug),
            variant::variant(slug).opposite()))
}

/// Partner subcommand
///
/// If `scheme` is `None`, uses the currently applied scheme.
pub fn partner(
    scheme: Option<&str>,
    base_dir: &Path,
    config_dir: &Path,
    json: bool,
    _verbose: bool,
) -> Result<()> {
    let source = match scheme {
        Some(s) => s.to_string(),
        None => get_current_scheme(base_dir)?,
    };
    let partner = resolve_partner(&source, base_dir, config_dir)?;
    if json {
        println!(
            "{}",
            serde_json::json!({
                "scheme": source,
                "partner": partner,
                "family": variant::family(&source),
            })
        );
    } else {
        println!("{}", partner);
    }
    Ok(())
}
