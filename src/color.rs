use std::env;
use std::io::IsTerminal;

/// Decide whether colored (truecolor) output should be produced.
///
/// Colors are disabled when the `NO_COLOR` environment variable is present and
/// non-empty (see <https://no-color.org>), or when standard output is not a
/// terminal (e.g. when piped into another program or redirected to a file).
pub fn colors_enabled() -> bool {
    let no_color = env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());
    !no_color && std::io::stdout().is_terminal()
}
