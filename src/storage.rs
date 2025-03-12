use chrono::Datelike;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// Creates a BufWriter for a file named "YYYY-MM-DD-00001.bin" in base/YYYY/MM/DD,
pub fn open_file_for_date(
    base_dir: &str,
    date: chrono::NaiveDate,
    file_index: usize,
    dir_perms: u32,
    file_perms: u32,
) -> io::Result<BufWriter<File>> {
    let year = date.year();
    let month = date.month();
    let day = date.day();

    // Build the directory path
    let dir_path: PathBuf = [
        base_dir,
        &year.to_string(),
        &format!("{:02}", month),
        &format!("{:02}", day),
    ]
    .iter()
    .collect();

    // Ensure the directory exists
    fs::create_dir_all(&dir_path)?;

    // Apply directory permissions
    let dir_metadata = fs::metadata(&dir_path)?;
    let mut directory_permissions = dir_metadata.permissions();
    directory_permissions.set_mode(dir_perms);
    fs::set_permissions(&dir_path, directory_permissions)?;

    // Build file name with 5-digit index
    let file_name = format!("{:04}-{:02}-{:02}-{:05}.bin", year, month, day, file_index);
    let file_path = dir_path.join(&file_name);

    // Open (or create) the file
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    // Apply file permissions
    let metadata = file.metadata()?;
    let mut perms = metadata.permissions();
    perms.set_mode(file_perms);
    fs::set_permissions(&file_path, perms)?;

    Ok(BufWriter::new(file))
}
