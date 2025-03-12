mod commands;
mod config;
mod storage;

use clap::Parser;
use commands::{Cli, Commands};
use config::Config;
use std::io;

fn main() -> io::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Load config from TOML, or use defaults if file is missing/invalid
    let config = Config::load_from_file("mouse-entropy.toml").unwrap_or_default();

    // Dispatch subcommands
    match cli.command {
        Commands::Start => commands::cmd_start(&config)?,
        Commands::Stop => commands::cmd_stop(&config)?,
        Commands::Clear => commands::cmd_clear(&config)?,
        Commands::Dump { date, output } => commands::cmd_dump(&config, date, output)?,
        Commands::Size => commands::cmd_size(&config)?,
    }

    Ok(())
}
