use clap::{crate_authors, crate_version, Arg, ArgAction, Command, ValueHint};

pub fn build_cli() -> Command {
    Command::new("flavours")
        .about("A simple way to manage and use base16 standard schemes and templates")
        .version(crate_version!())
        .author(crate_authors!())
        .propagate_version(true)
        .disable_help_subcommand(true)
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .help("Be more verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("config")
                .help("Specify a configuration file (Defaults to ~/.config/flavours/config.toml on Linux)")
                .long("config")
                .short('c')
                .value_name("FILE")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("directory")
                .help("Specify a data directory (Defaults to ~/.local/share/flavours on Linux)")
                .long("directory")
                .short('d')
                .value_name("DIRECTORY")
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("completions")
                .hide(true)
                .help("Generates completion for given shell, outputs to stdout")
                .long("completions")
                .value_parser(["bash", "elvish", "fish", "powershell", "zsh"]),
        )
        .subcommand(
            Command::new("current")
                .about("Prints last applied scheme name")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("json")
                        .help("Output the current scheme as JSON")
                        .long("json")
                        .short('j')
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Prints a list with all matching schemes")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("templates")
                        .help("List templates instead of schemes")
                        .long("templates")
                        .short('t')
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("pattern")
                        .help("Scheme name or glob pattern to match when listing scheme(s). If ommited, defaults to * (all installed schemes).")
                        .value_hint(ValueHint::Other)
                        .num_args(0..),
                )
                .arg(
                    Arg::new("lines")
                        .help("Print each scheme on its own line")
                        .long("lines")
                        .short('l')
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("json")
                        .help("Output the list as a JSON array")
                        .long("json")
                        .short('j')
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("long")
                        .help("With --json, emit enriched objects [{slug, family, mode}] instead of a plain string array")
                        .long("long")
                        .short('L')
                        .action(ArgAction::SetTrue)
                        .requires("json"),
                ),
        )
        .subcommand(
            Command::new("families")
                .about("Prints unique scheme families (name shared between light/dark variants)")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("json")
                        .help("Output as JSON: [{family, dark, light}]")
                        .long("json")
                        .short('j')
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Shows scheme colors for all schemes matching pattern. Optionally uses truecolor")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("pattern")
                        .help("Scheme name or glob pattern to match when showing scheme(s). If ommited, defaults to * (all installed schemes).")
                        .value_hint(ValueHint::Other)
                        .num_args(0..),
                )
                .arg(
                    Arg::new("raw")
                        .help("Don't pretty print the colors.")
                        .long("raw")
                        .short('r')
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("json")
                        .help("Output scheme information as JSON")
                        .long("json")
                        .short('j')
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a scheme based on an image")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("mode")
                        .help("Whether to generate a dark or light scheme")
                        .value_parser(["dark", "light"])
                        .required(true)
                        .value_hint(ValueHint::Other),
                )
                .arg(
                    Arg::new("file")
                        .help("Which image file to use. Use '-' or --stdin to read the image from standard input.")
                        .required(false)
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("stdin")
                        .help("Read the image from standard input instead of a file.")
                        .long("stdin")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("slug")
                        .long("slug")
                        .short('s')
                        .help("Scheme slug (the name you specify when applying schemes) to output to. If ommited, defaults to 'generated'")
                        .value_name("slug")
                        .value_hint(ValueHint::Other),
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .help("Scheme display name (can include spaces and capitalization) to write, defaults to 'Generated'")
                        .value_name("name")
                        .value_hint(ValueHint::Other),
                )
                .arg(
                    Arg::new("author")
                        .long("author")
                        .short('a')
                        .help("Scheme author info (name, email, etc) to write, defaults to 'Flavours'")
                        .value_name("author")
                        .value_hint(ValueHint::Other),
                )
                .arg(
                    Arg::new("stdout")
                        .help("Outputs scheme to stdout instead of writing it to a file.")
                        .long("stdout")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("apply")
                .about("Applies scheme, according to user configuration")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("pattern")
                        .help("Scheme to be applied, supports glob. If more than one is specified (or if glob pattern matched more than one), chooses one randomly. If ommited, defaults to * (all installed schemes).")
                        .value_hint(ValueHint::Other)
                        .num_args(0..),
                )
                .arg(
                    Arg::new("light")
                        .help("Skip running heavier hooks (entries marked 'light=false')")
                        .long("light")
                        .short('l')
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("stdin")
                        .help("Reads scheme from stdin instead of from flavours directory.")
                        .long("stdin")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("mode"),
                )
                .arg(
                    Arg::new("mode")
                        .help("Apply the light/dark partner of the current scheme (same family). Use 'toggle' to switch to the opposite variant, or 'dark'/'light' to force one.")
                        .long("mode")
                        .short('m')
                        .value_parser(["dark", "light", "toggle"])
                        .value_name("MODE")
                        .conflicts_with("pattern")
                        .conflicts_with("stdin"),
                ),
        )
        .subcommand(
            Command::new("partner")
                .about("Prints the light/dark partner of the given scheme (or the currently applied one)")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("scheme")
                        .help("Scheme slug to find the partner of. Defaults to the currently applied scheme.")
                        .value_hint(ValueHint::Other),
                )
                .arg(
                    Arg::new("json")
                        .help("Output as JSON: {scheme, partner, family}")
                        .long("json")
                        .short('j')
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("toggle")
                .about("Cycles through the given schemes, applying the next one after the currently applied scheme")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("pattern")
                        .help("Schemes to cycle through, in order. Supports glob patterns, which are expanded (and sorted) into concrete schemes.")
                        .value_hint(ValueHint::Other)
                        .required(true)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("light")
                        .help("Skip running heavier hooks (entries marked 'light=false')")
                        .long("light")
                        .short('l')
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Downloads schemes, templates, or updates their lists (from repos specified in sources.yml)")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("operation")
                        .help("Update sources lists from repositories or (re)download schemes/templates specified in the lists. Default repositories for lists, and the lists themselves, can be manually changed.")
                        .required(true)
                        .value_parser(["lists", "schemes", "templates", "all"]),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Builds a template with given scheme, outputs to stdout")
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .arg(
                    Arg::new("scheme")
                        .help("Path to scheme file.")
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("template")
                        .help("Path to template file.")
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
}
