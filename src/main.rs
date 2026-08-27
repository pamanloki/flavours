use anyhow::{anyhow, Context, Result};
use base16_color_scheme::{
    scheme::{BaseIndex, RgbColor},
    Scheme,
};
use dirs::{data_dir, preference_dir};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::env;
use std::io::Read;
use std::path::Path;

use flavours::operations::{apply, build, current, generate, info, list, list_templates, toggle, update};
use flavours::{cli, completions};

use std::fs::{create_dir_all, write};

/// Collect the values of a variadic argument into a `Vec<&str>`, defaulting to
/// the wildcard pattern when none were supplied.
fn patterns_or_wildcard(matches: &clap::ArgMatches) -> Vec<&str> {
    match matches.get_many::<String>("pattern") {
        Some(values) => values.map(String::as_str).collect(),
        None => vec!["*"],
    }
}

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    // Completions flag
    if let Some(shell) = matches.get_one::<String>("completions") {
        return completions::completions(Some(shell.as_str()));
    };

    // Flavours data directory
    let flavours_dir = match matches.get_one::<String>("directory") {
        // User supplied
        Some(argument) => Path::new(argument)
            .canonicalize()
            .with_context(|| "Invalid data directory supplied on argument")?,
        // If not supplied
        None => {
            // Try to get from env var
            match env::var("FLAVOURS_DATA_DIRECTORY") {
                Ok(path) => Path::new(&path)
                    .canonicalize()
                    .with_context(|| "Invalid data directory supplied on env var")?,
                // Use default instead
                Err(_) => data_dir()
                    .ok_or_else(|| anyhow!("Error getting default data directory"))?
                    .join("flavours"),
            }
        }
    };

    let flavours_config_dir = preference_dir()
        .ok_or_else(|| anyhow!("Error getting default config directory"))?
        .join("flavours");

    // Flavours config file
    let flavours_config = match matches.get_one::<String>("config") {
        // User supplied
        Some(path) => Path::new(path)
            .canonicalize()
            .with_context(|| "Invalid config file supplied on argument")?,
        // If not supplied
        None => {
            // Try to get from env var
            match env::var("FLAVOURS_CONFIG_FILE") {
                Ok(path) => Path::new(&path)
                    .canonicalize()
                    .with_context(|| "Invalid config file supplied on env var")?,
                // Use default instead
                Err(_) => flavours_config_dir.join("config.toml"),
            }
        }
    };

    // Should we be verbose?
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("Using directory: {:?}", flavours_dir);
        println!("Using config file: {:?}", flavours_config);
    };

    // Check which subcommand was used
    match matches.subcommand() {
        Some(("current", sub_matches)) => {
            current::current(&flavours_dir, sub_matches.get_flag("json"), verbose)
        }

        Some(("apply", sub_matches)) => {
            let patterns = patterns_or_wildcard(sub_matches);
            let light = sub_matches.get_flag("light");
            let from_stdin = sub_matches.get_flag("stdin");
            apply::apply(
                patterns,
                &flavours_dir,
                &flavours_config_dir,
                &flavours_config,
                light,
                from_stdin,
                verbose,
            )
        }

        Some(("toggle", sub_matches)) => {
            let patterns = patterns_or_wildcard(sub_matches);
            let light = sub_matches.get_flag("light");
            toggle::toggle(
                patterns,
                &flavours_dir,
                &flavours_config_dir,
                &flavours_config,
                light,
                verbose,
            )
        }

        Some(("build", sub_matches)) => {
            // Get file paths
            let scheme_file = sub_matches
                .get_one::<String>("scheme")
                .ok_or_else(|| anyhow!("You must specify a scheme file"))?;
            let template_file = sub_matches
                .get_one::<String>("template")
                .ok_or_else(|| anyhow!("You must specify a template file"))?;
            build::build(Path::new(scheme_file), Path::new(template_file))
        }

        Some(("list", sub_matches)) => {
            let patterns = patterns_or_wildcard(sub_matches);
            let lines = sub_matches.get_flag("lines");
            let json = sub_matches.get_flag("json");

            if sub_matches.get_flag("templates") {
                list_templates::list(
                    patterns,
                    &flavours_dir,
                    &flavours_config_dir,
                    verbose,
                    lines,
                    json,
                )
            } else {
                list::list(
                    patterns,
                    &flavours_dir,
                    &flavours_config_dir,
                    verbose,
                    lines,
                    json,
                )
            }
        }

        Some(("update", sub_matches)) => {
            let operation = sub_matches
                .get_one::<String>("operation")
                .ok_or_else(|| anyhow!("Invalid operation"))?;
            update::update(operation, &flavours_dir, verbose, &flavours_config)
        }

        Some(("info", sub_matches)) => {
            let patterns = patterns_or_wildcard(sub_matches);
            let raw = sub_matches.get_flag("raw");
            let json = sub_matches.get_flag("json");
            info::info(patterns, &flavours_dir, &flavours_config_dir, raw, json)
        }

        Some(("generate", sub_matches)) => {
            let slug = sub_matches
                .get_one::<String>("slug")
                .map_or("generated", String::as_str)
                .into();
            let name = sub_matches
                .get_one::<String>("name")
                .map_or("Generated", String::as_str)
                .into();
            let author = sub_matches
                .get_one::<String>("author")
                .map_or("Flavours", String::as_str)
                .into();

            let file = sub_matches.get_one::<String>("file").map(String::as_str);
            let from_stdin = sub_matches.get_flag("stdin") || file == Some("-");

            // Load the image, either from stdin or from the given file
            let image = if from_stdin {
                let mut buffer = Vec::new();
                std::io::stdin()
                    .lock()
                    .read_to_end(&mut buffer)
                    .with_context(|| "Couldn't read image from stdin")?;
                image::load_from_memory(&buffer)
                    .with_context(|| "Couldn't decode image read from stdin")?
            } else {
                let file = file
                    .ok_or_else(|| anyhow!("No image file specified (use a file path, '-', or --stdin)"))?;
                let path = Path::new(file)
                    .canonicalize()
                    .with_context(|| "Invalid image file supplied")?;
                image::open(&path).with_context(|| format!("Couldn't open image {:?}", path))?
            };

            let mode = match sub_matches.get_one::<String>("mode").map(String::as_str) {
                Some("dark") => Ok(generate::Mode::Dark),
                Some("light") => Ok(generate::Mode::Light),
                _ => Err(anyhow!("No valid mode specified")),
            }?;

            let to_stdout = sub_matches.get_flag("stdout");

            let colors = generate::generate(image, mode, verbose)?;
            let scheme = Scheme {
                scheme: name,
                slug,
                author,
                colors: colors
                    .into_iter()
                    .enumerate()
                    .map(|(index, color)| {
                        let mut rgb_color = [0u8; 3];
                        hex::decode_to_slice(color, &mut rgb_color)?;
                        Ok((BaseIndex(index.try_into()?), RgbColor(rgb_color)))
                    })
                    .collect::<Result<BTreeMap<_, _>>>()?,
            };

            if to_stdout {
                print!("{}", serde_yaml::to_string(&scheme)?);
            } else {
                let path = flavours_dir.join("base16").join("schemes").join("generated");
                if !path.exists() {
                    create_dir_all(&path)
                        .with_context(|| format!("Couldn't create directory {:?}", &path))?;
                }
                let file_path = &path.join(format!("{}.yaml", &scheme.slug));
                write(file_path, serde_yaml::to_string(&scheme)?)
                    .with_context(|| format!("Couldn't write scheme file at {:?}", path))?;
            }
            Ok(())
        }
        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
