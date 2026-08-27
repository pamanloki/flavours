use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

/// Get current scheme
///
/// * `dir` - flavours data directory
pub fn get_current_scheme(dir: &Path) -> Result<String> {
    // File that stores last used scheme
    let file_path = &dir.join("lastscheme");
    // Try to open it
    let scheme: String = read_to_string(file_path)
        .with_context(|| "Failed to read last scheme file. Try applying first.")?
        .split_whitespace()
        .collect();

    if scheme.is_empty() {
        Err(anyhow!(
            "Failed to read last scheme from file. Try applying first."
        ))
    } else {
        Ok(scheme)
    }
}

/// Current subcommand
///
/// * `base_dir` - flavours data directory
/// * `json` - Should we output the scheme as JSON?
/// * `verbose` - Should we be verbose (unused atm)
pub fn current(base_dir: &Path, json: bool, _verbose: bool) -> Result<()> {
    let scheme = get_current_scheme(base_dir)?;
    if json {
        println!("{}", serde_json::json!({ "scheme": scheme }));
    } else {
        println!("{}", scheme);
    }
    Ok(())
}
