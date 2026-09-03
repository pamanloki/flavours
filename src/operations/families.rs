use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::path::Path;

use crate::find::find_schemes;
use crate::variant;

#[derive(Default)]
struct FamilyEntry {
    dark: Option<String>,
    light: Option<String>,
}

/// Families subcommand
///
/// Prints unique family names derived from installed scheme slugs. In JSON
/// mode, each entry also carries the concrete `dark`/`light` slugs it maps
/// to (either may be `null` when only one variant is installed).
///
/// * `base_dir` - flavours base data dir
/// * `config_dir` - flavours config dir
/// * `json` - Should we output as JSON?
/// * `verbose` - Should we be verbose?
pub fn families(
    base_dir: &Path,
    config_dir: &Path,
    json: bool,
    _verbose: bool,
) -> Result<()> {
    let found = find_schemes("*", base_dir, config_dir)?;
    let mut slugs: Vec<String> = found
        .iter()
        .filter_map(|path| path.file_stem())
        .filter_map(|stem| stem.to_str())
        .map(String::from)
        .collect();
    slugs.sort();
    slugs.dedup();

    if slugs.is_empty() {
        return Err(anyhow!(
            "No schemes installed. Run 'flavours update all' first."
        ));
    }

    let mut families: BTreeMap<String, FamilyEntry> = BTreeMap::new();
    for slug in &slugs {
        let fam = variant::family(slug);
        let entry = families.entry(fam).or_default();
        match variant::variant(slug) {
            variant::Variant::Dark => {
                if entry.dark.is_none() {
                    entry.dark = Some(slug.clone());
                }
            }
            variant::Variant::Light => {
                if entry.light.is_none() {
                    entry.light = Some(slug.clone());
                }
            }
        }
    }

    if json {
        let entries: Vec<_> = families
            .iter()
            .map(|(name, e)| {
                serde_json::json!({
                    "family": name,
                    "dark": e.dark,
                    "light": e.light,
                })
            })
            .collect();
        println!("{}", serde_json::to_string(&entries)?);
    } else {
        for name in families.keys() {
            println!("{}", name);
        }
    }
    Ok(())
}
