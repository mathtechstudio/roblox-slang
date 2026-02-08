mod cli;
mod config;
mod generator;
mod migrator;
mod parser;
mod utils;
mod validator;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;
use utils::validation;

#[derive(Parser)]
#[command(name = "roblox-slang")]
#[command(version)]
#[command(about = "Type-safe internationalization for Roblox")]
#[command(
    long_about = "Roblox Slang is a CLI tool that generates type-safe Luau code from translation files.\n\
                        Write translations in JSON/YAML, generate type-safe code with autocomplete support.\n\n\
                        For more information, visit: https://github.com/mathtechstudio/roblox-slang"
)]
#[command(author = "Iqbal Fauzi <iqbalfauzien@proton.me>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Roblox Slang project
    ///
    /// Creates a new project with default configuration file and translation directory.
    /// Use --with-overrides to also create an overrides.yaml template for A/B testing.
    Init {
        /// Create example overrides.yaml file for translation overrides
        #[arg(long, help = "Create overrides.yaml template")]
        with_overrides: bool,
    },

    /// Build translations and generate Luau code
    ///
    /// Parses translation files (JSON/YAML) and generates type-safe Luau code.
    /// Outputs: Translations.lua, type definitions, and CSV for Roblox Cloud.
    Build {
        /// Watch for file changes and rebuild automatically
        #[arg(short, long, help = "Enable watch mode (auto-rebuild on changes)")]
        watch: bool,
    },

    /// Import translations from a Roblox CSV file
    ///
    /// Converts Roblox Cloud CSV format to JSON translation files.
    /// Useful for migrating existing translations or syncing with Roblox Cloud.
    Import {
        /// Path to the CSV file to import
        #[arg(value_name = "CSV_FILE", help = "Path to Roblox CSV file")]
        csv_file: String,
    },

    /// Validate translations for errors and inconsistencies
    ///
    /// Checks for missing translations, unused keys, conflicts, and coverage.
    /// Use --all to run all checks at once.
    Validate {
        /// Check for missing translations across locales
        #[arg(long, help = "Check for missing translations")]
        missing: bool,

        /// Check for unused translation keys in source code
        #[arg(long, help = "Check for unused keys")]
        unused: bool,

        /// Check for duplicate keys or conflicts
        #[arg(long, help = "Check for conflicts")]
        conflicts: bool,

        /// Show translation coverage report per locale
        #[arg(long, help = "Show coverage report")]
        coverage: bool,

        /// Source directory to scan for unused keys
        #[arg(long, value_name = "DIR", help = "Source directory to scan")]
        source: Option<String>,

        /// Run all validation checks
        #[arg(long, help = "Run all checks")]
        all: bool,
    },

    /// Migrate translations from another format
    ///
    /// Converts translations from other formats (custom-json, gettext) to Roblox Slang format.
    /// Supports key transformation strategies for compatibility.
    Migrate {
        /// Format to migrate from
        #[arg(
            long,
            value_name = "FORMAT",
            help = "Source format (custom-json, gettext)"
        )]
        from: String,

        /// Input file path
        #[arg(long, value_name = "FILE", help = "Input file path")]
        input: String,

        /// Output file path
        #[arg(long, value_name = "FILE", help = "Output file path")]
        output: String,

        /// Key transformation strategy
        #[arg(
            long,
            value_name = "TRANSFORM",
            help = "Key transformation (snake-to-camel, upper-to-lower, dot-to-nested, none)"
        )]
        transform: Option<String>,
    },
}

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { with_overrides } => {
            cli::init(with_overrides)?;
        }
        Commands::Build { watch } => {
            let config_path = Path::new("slang-roblox.yaml");

            if watch {
                cli::watch(config_path)?;
            } else {
                cli::build(config_path)?;
            }
        }
        Commands::Import { csv_file } => {
            let csv_path = Path::new(&csv_file);

            // Validate file path
            validation::validate_safe_path(csv_path)?;
            validation::validate_file_exists(csv_path, "CSV file")?;

            let config_path = Path::new("slang-roblox.yaml");
            cli::import_csv(csv_path, config_path)?;
        }
        Commands::Validate {
            missing,
            unused,
            conflicts,
            coverage,
            source,
            all,
        } => {
            let config_path = Path::new("slang-roblox.yaml");

            // If --all is specified, enable all checks
            let check_missing = all || missing;
            let check_unused = all || unused;
            let check_conflicts = all || conflicts;
            let show_coverage = all || coverage;

            let source_dir = if let Some(ref s) = source {
                let path = Path::new(s.as_str());
                validation::validate_safe_path(path)?;
                validation::validate_directory_exists(path, "source directory")?;
                Some(path)
            } else {
                None
            };

            cli::validate(
                config_path,
                check_missing,
                check_unused,
                check_conflicts,
                show_coverage,
                source_dir,
            )?;
        }
        Commands::Migrate {
            from,
            input,
            output,
            transform,
        } => {
            let input_path = Path::new(&input);
            let output_path = Path::new(&output);

            // Validate file paths
            validation::validate_safe_path(input_path)?;
            validation::validate_file_exists(input_path, "input file")?;
            validation::validate_safe_path(output_path)?;

            let transform_str = transform.as_deref();

            cli::migrate(&from, input_path, output_path, transform_str)?;
        }
    }

    Ok(())
}
