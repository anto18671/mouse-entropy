// src/commands.rs
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::storage::open_file_for_date;
use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};

/// CLI definition
#[derive(Parser)]
#[command(name = "mouse-capture")]
#[command(about = "A CLI to start, stop, clear, and dump mouse data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Start capturing mouse data in the foreground
    Start,
    /// Stop capturing (stub; real implementation depends on daemonization)
    Stop,
    /// Clear all recorded data
    Clear,
    /// Dump data as CSV (optionally for one specific YYYY-MM-DD)
    Dump {
        /// Optional day to dump: YYYY-MM-DD
        date: Option<String>,
        /// Optional CSV output file (otherwise stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Summarize the size of all recorded data
    Size,
}

// ---------------------------
// Commands
// ---------------------------

pub fn cmd_start(config: &Config) -> io::Result<()> {
    println!("Starting capture on device: {}", config.storage.device_path);

    let mut mouse_file = File::open(&config.storage.device_path).unwrap_or_else(|_| {
        panic!(
            "Failed to open {}. Are you root or in the 'input' group?",
            config.storage.device_path
        )
    });

    let mut current_date = Local::now().date_naive();
    let mut file_index = 1;
    let mut writer = open_file_for_date(
        &config.storage.base_directory,
        current_date,
        file_index,
        config.storage.directory_permissions,
        config.storage.file_permissions,
    )?;

    let mut bytes_in_current_file = 0u64;
    let max_file_size_bytes = config.storage.max_file_size_mb * 1024 * 1024;
    let mut mouse_buf = [0u8; 3];

    println!("Capturing (Ctrl+C to stop)...");

    loop {
        let today = Local::now().date_naive();
        if today != current_date {
            writer.flush()?;
            file_index = 1;
            bytes_in_current_file = 0;
            current_date = today;
            writer = open_file_for_date(
                &config.storage.base_directory,
                current_date,
                file_index,
                config.storage.directory_permissions,
                config.storage.file_permissions,
            )?;
        }

        match mouse_file.read_exact(&mut mouse_buf) {
            Ok(_) => {
                let button_byte = mouse_buf[0];
                let dx = mouse_buf[1];
                let dy = mouse_buf[2];

                if config.storage.store_4_bytes {
                    writer.write_all(&[button_byte, dx, dy, 0])?;
                    bytes_in_current_file += 4;
                } else {
                    writer.write_all(&[button_byte, dx, dy])?;
                    bytes_in_current_file += 3;
                }

                if bytes_in_current_file >= max_file_size_bytes {
                    writer.flush()?;
                    file_index += 1;
                    bytes_in_current_file = 0;
                    writer = open_file_for_date(
                        &config.storage.base_directory,
                        current_date,
                        file_index,
                        config.storage.directory_permissions,
                        config.storage.file_permissions,
                    )?;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", config.storage.device_path, e);
                break;
            }
        }
    }

    Ok(())
}

pub fn cmd_stop(_config: &Config) -> io::Result<()> {
    // Stub: In a real daemon approach, you'd have a PID file to kill, etc.
    println!("Stop not implemented (use Ctrl+C or manage via systemd/daemon).");
    Ok(())
}

pub fn cmd_clear(config: &Config) -> io::Result<()> {
    let base = &config.storage.base_directory;
    println!("Clearing all data in '{}'.", base);

    if Path::new(base).exists() {
        fs::remove_dir_all(base)?;
        println!("Removed '{}'.", base);
    } else {
        println!("Directory '{}' does not exist.", base);
    }
    Ok(())
}

pub fn cmd_dump(config: &Config, date: Option<String>, output: Option<String>) -> io::Result<()> {
    match date {
        Some(date_str) => {
            // ------------------------------------
            // 1) Dump for the single YYYY-MM-DD
            // ------------------------------------
            let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid date format (YYYY-MM-DD)",
                )
            })?;

            let mut out_writer: Box<dyn Write> = if let Some(path) = output {
                Box::new(File::create(path)?)
            } else {
                Box::new(io::stdout())
            };

            // Write CSV header
            writeln!(out_writer, "button,dx,dy")?;

            dump_one_day(config, parsed_date, &mut out_writer)?;
        }
        None => {
            // ------------------------------------
            // 2) Dump *all* days found under base/YYYY/MM/DD
            // ------------------------------------
            let mut out_writer: Box<dyn Write> = if let Some(path) = output {
                Box::new(File::create(path)?)
            } else {
                Box::new(io::stdout())
            };

            // Write CSV header
            writeln!(out_writer, "button,dx,dy")?;

            let base = &config.storage.base_directory;
            let base_path = Path::new(base);

            // If the base directory doesn't exist, nothing to dump
            if !base_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Directory '{}' does not exist.", base),
                ));
            }

            // For each "year" folder (YYYY)
            for year_entry in fs::read_dir(base_path)? {
                let year_entry = match year_entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                if !year_entry.file_type()?.is_dir() {
                    continue;
                }

                let year_path = year_entry.path();
                let year_str = year_entry.file_name().to_string_lossy().to_string();
                let year: i32 = match year_str.parse() {
                    Ok(y) => y,
                    Err(_) => continue, // skip non-numeric
                };

                // For each "month" folder (MM)
                for month_entry in fs::read_dir(&year_path)? {
                    let month_entry = match month_entry {
                        Ok(entry) => entry,
                        Err(_) => continue,
                    };
                    if !month_entry.file_type()?.is_dir() {
                        continue;
                    }

                    let month_path = month_entry.path();
                    let month_str = month_entry.file_name().to_string_lossy().to_string();
                    let month: u32 = match month_str.parse() {
                        Ok(m) => m,
                        Err(_) => continue, // skip non-numeric
                    };

                    // For each "day" folder (DD)
                    for day_entry in fs::read_dir(&month_path)? {
                        let day_entry = match day_entry {
                            Ok(entry) => entry,
                            Err(_) => continue,
                        };
                        if !day_entry.file_type()?.is_dir() {
                            continue;
                        }

                        let day_str = day_entry.file_name().to_string_lossy().to_string();
                        let day: u32 = match day_str.parse() {
                            Ok(d) => d,
                            Err(_) => continue, // skip non-numeric
                        };

                        // Construct a NaiveDate for that folder
                        let parsed_date = match NaiveDate::from_ymd_opt(year, month, day) {
                            Some(dt) => dt,
                            None => continue,
                        };

                        // Dump that single day
                        dump_one_day(config, parsed_date, &mut out_writer)?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Dump a single day worth of data (YYYY-MM-DD) by looking for .bin files and writing CSV.
fn dump_one_day<W: Write>(
    config: &Config,
    parsed_date: NaiveDate,
    out_writer: &mut W,
) -> io::Result<()> {
    // The folder: base/YYYY/MM/DD
    let dir_path = {
        let y = parsed_date.year();
        let m = parsed_date.month();
        let d = parsed_date.day();
        [
            &config.storage.base_directory,
            &y.to_string(),
            &format!("{:02}", m),
            &format!("{:02}", d),
        ]
        .iter()
        .collect::<PathBuf>()
    };

    if !dir_path.exists() {
        // No data for that day, skip
        return Ok(());
    }

    // Find all chunk files named "YYYY-MM-DD-xxxxx.bin"
    let pattern = format!(
        "{:04}-{:02}-{:02}-",
        parsed_date.year(),
        parsed_date.month(),
        parsed_date.day()
    );

    let mut chunk_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(&pattern) && name_str.ends_with(".bin") {
                chunk_files.push(entry.path());
            }
        }
    }
    chunk_files.sort(); // sort by name => 00001, 00002, etc.

    if chunk_files.is_empty() {
        // There's a day folder but no .bin files, just skip
        return Ok(());
    }

    let is_4_bytes = config.storage.store_4_bytes;
    let record_size = if is_4_bytes { 4 } else { 3 };

    // Read each chunk and output CSV lines
    for path in &chunk_files {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; record_size];

        loop {
            match reader.read_exact(&mut buffer) {
                Ok(_) => {
                    let button = buffer[0];
                    let dx = buffer[1] as i8;
                    let dy = buffer[2] as i8;
                    // buffer[3] if present is just padding

                    writeln!(out_writer, "{},{},{}", button, dx, dy)?;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}

pub fn cmd_size(config: &Config) -> io::Result<()> {
    let base = &config.storage.base_directory;
    let base_path = Path::new(base);

    if !base_path.exists() {
        println!("Base directory '{}' does not exist.", base);
        return Ok(());
    }

    let mut total_all: u64 = 0;

    // Read all entries under base_directory: typically "YYYY" folders
    for year_entry in fs::read_dir(base_path)? {
        let year_entry = match year_entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        // Ensure it's a directory
        if !year_entry.file_type()?.is_dir() {
            continue;
        }

        let year_str = year_entry.file_name().to_string_lossy().to_string();
        // Try to parse as i32; skip if not parseable
        let year: i32 = match year_str.parse() {
            Ok(y) => y,
            Err(_) => continue,
        };

        let year_path = year_entry.path();
        let mut total_year = 0u64;

        // Inside each year folder, expect "MM" sub-folders
        for month_entry in fs::read_dir(&year_path)? {
            let month_entry = match month_entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            if !month_entry.file_type()?.is_dir() {
                continue;
            }

            let month_str = month_entry.file_name().to_string_lossy().to_string();
            let month: u32 = match month_str.parse() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let month_path = month_entry.path();
            let mut total_month = 0u64;

            // Inside each month folder, expect "DD" sub-folders
            for day_entry in fs::read_dir(&month_path)? {
                let day_entry = match day_entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };

                if !day_entry.file_type()?.is_dir() {
                    continue;
                }

                let day_str = day_entry.file_name().to_string_lossy().to_string();
                let day: u32 = match day_str.parse() {
                    Ok(d) => d,
                    Err(_) => continue,
                };

                let day_path = day_entry.path();
                let mut total_day = 0u64;

                // Sum all files in this "day" folder
                for file_entry in fs::read_dir(&day_path)? {
                    let file_entry = match file_entry {
                        Ok(fe) => fe,
                        Err(_) => continue,
                    };

                    if file_entry.file_type()?.is_file() {
                        let metadata = file_entry.metadata()?;
                        total_day += metadata.len();
                    }
                }

                println!(
                    "Year {} Month {:02} Day {:02} => {} bytes",
                    year, month, day, total_day
                );
                total_month += total_day;
            }

            println!("Year {} Month {:02} => {} bytes", year, month, total_month);
            total_year += total_month;
        }

        println!("Year {} => {} bytes", year, total_year);
        total_all += total_year;
    }

    println!("Total across all years/months/days => {} bytes", total_all);

    Ok(())
}
