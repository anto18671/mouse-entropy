# Mouse Entropy

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#)

---

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
  - [Building From Source](#building-from-source)
  - [Arch Linux Package Installation](#arch-linux-package-installation)
- [Configuration](#configuration)
  - [TOML Configuration File](#toml-configuration-file)
  - [Permissions](#permissions)
- [Usage](#usage)
  - [Commands](#commands)
    - [Screenshot of the Help Command](#screenshot-of-the-help-command)
    - [Explanation of Each Command](#explanation-of-each-command)
  - [Daemon / Systemd Service](#daemon--systemd-service)
- [Data Storage Structure](#data-storage-structure)
- [Examples](#examples)
  - [Quick Dump Example](#quick-dump-example)
  - [Clearing Data](#clearing-data)
  - [Viewing File Sizes](#viewing-file-sizes)
- [Logging & Debugging](#logging--debugging)
- [FAQ](#faq)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)
- [Author / Maintainer](#author--maintainer)
- [Acknowledgments](#acknowledgments)

---

## Introduction

**Mouse Entropy** is a command-line tool designed primarily for Arch-based Linux systems to capture and store raw mouse event data. It can be useful for various purposes, such as:

1. Generating entropy from user mouse movements.
2. Analyzing mouse usage patterns.
3. Collecting mouse input for testing or further research on human-computer interaction.

The tool reads events from the Linux mouse input device (`/dev/input/mice` by default) in real time and saves them to binary files. Data can then be easily dumped to CSV format for offline analysis.

---

## Features

- **Real-time capturing** of mouse data from `/dev/input/mice`.
- **Configurable storage** (directory structure, permissions, file sizes).
- **Easy-to-use CLI** with subcommands for starting, stopping, clearing, dumping, and summarizing data sizes.
- **Automatic file rotation** after configurable max file size is reached, preventing single large files.
- **Daily folder structure** for easy organization, e.g. `data/YYYY/MM/DD`.
- **CSV export** for offline analysis.
- **Arch Linux systemd service** for hands-off, daemonized operation.

---

## Requirements

- **Rust** (if building from source)
- **Cargo** (if building from source)
- **Arch Linux** or an Arch-based distribution (for the provided `.service` file and PKGBUILD usage).
- **Privileges**: You typically need to run as `root` or be a member of the `input` group to access `/dev/input/mice`.

---

## Installation

### Building From Source

1. Clone this repository:

   ```bash
   git clone https://github.com/anto18671/mouse-entropy.git
   ```

2. Navigate to the project folder:

   ```bash
   cd mouse-entropy
   ```

3. Build in release mode:

   ```bash
   cargo build --release
   ```

4. Optionally, install it system-wide:

   ```bash
   sudo install -Dm755 target/release/mouse-entropy /usr/bin/mouse-entropy
   ```

5. Copy or move the default config file:

   ```bash
   sudo mkdir -p /etc/mouse-entropy
   sudo cp mouse-entropy.toml /etc/mouse-entropy/
   ```

### Arch Linux Package Installation

A `PKGBUILD` file is provided for Arch-based systems. You can build and install it using your favorite AUR helper or with the standard `makepkg` workflow.

1. Download (or clone) the PKGBUILD and related files (`mouse-entropy.install`, `.service`, `.toml`, etc.).
2. Run:

   ```bash
   makepkg -si
   ```

   This command will:

   - Build the package.
   - Prompt to install it (`-i`).
   - Place the binary into `/usr/bin/mouse-entropy`.
   - Place the default config into `/etc/mouse-entropy/mouse-entropy.toml`.
   - Place the systemd service into `/usr/lib/systemd/system/mouse-entropy.service`.

Once installed, you have a fully functional `mouse-entropy` command-line tool.

---

## Configuration

The default configuration file is stored in:

```
/etc/mouse-entropy/mouse-entropy.toml
```

### TOML Configuration File

Below is an example TOML file with annotations:

```toml
[storage]
base_directory = "data"             # Directory where recorded data is stored
directory_permissions = 448         # Decimal equivalent of 0o700 => drwx------
file_permissions = 384              # Decimal equivalent of 0o600 => -rw-------
max_file_size_mb = 2                # Maximum size of each data chunk file
store_4_bytes = false               # If true => store 4 bytes per event ([button, dx, dy, 0])
device_path = "/dev/input/mice"     # Path to the mouse device file
```

**Key Points:**

- `base_directory`: Where all data is stored. Can be absolute or relative.
- `directory_permissions`: The directory permissions in decimal format. `448` = `0o700`.
- `file_permissions`: The file permissions in decimal format. `384` = `0o600`.
- `max_file_size_mb`: Once a data file hits this size, a new file is created.
- `store_4_bytes`: Whether to store 3 or 4 bytes per event. The 4th byte is a zero-byte placeholder.
- `device_path`: The input device. On many Linux systems, `/dev/input/mice` is used for aggregated mouse movements.

### Permissions

- If using the default `device_path = "/dev/input/mice"`, you need read access to this special file.
  - Typically, you must run `mouse-entropy` as `root` or be in the `input` group.
- The stored data and directories are created with the specified permissions (e.g., `0o700` for directories and `0o600` for files by default).

---

## Usage

`mouse-entropy` is a command-line tool with multiple subcommands. Once installed, you can type `mouse-entropy --help` or just run it to see usage instructions.

### Commands

Below is a screenshot of the help command output, followed by a detailed explanation for each subcommand.

#### Screenshot of the Help Command

```
$ mouse-entropy --help
```

```
Usage: mouse-entropy <COMMAND>

Commands:
  start    Start capturing mouse data in the foreground
  stop     Stop capturing (stub; real implementation depends on daemonization)
  clear    Clear all recorded data
  dump     Dump data as CSV (optionally for one specific YYYY-MM-DD)
  size     Summarize the size of all recorded data
  help     Print this message or the help of the given subcommand(s)
```

_(Imagine a screenshot image here in your final README, showing the same content. Replace this code block with an actual screenshot or keep it as text if you prefer.)_

#### Explanation of Each Command

1. **start**

   - Launches real-time capturing of mouse events from the configured `device_path`.
   - Stores the binary event data into daily subfolders, splitting files when they exceed `max_file_size_mb`.
   - Output continues until you interrupt (Ctrl + C) or otherwise kill the process.

2. **stop**

   - A stub subcommand. In future or alternative daemonized approaches, this could stop a running service or kill a process by PID file. Currently, use Ctrl + C or systemd to stop.

3. **clear**

   - Removes **all** recorded data in the configured `base_directory`.
   - **Warning:** This is irreversible.

4. **dump**

   - Converts stored binary data to CSV output, either to `stdout` or a specified file.
   - Optionally specify a date (`YYYY-MM-DD`) to only dump a single day's data.
     - Example: `mouse-entropy dump --date 2025-01-01 --output data_2025_01_01.csv`
   - If no date is specified, **all** days in `base_directory` are read and dumped.

5. **size**
   - Recursively walks the storage directories to calculate total usage by day, month, and year.
   - Shows a summary of how large your stored data has become.

---

### Daemon / Systemd Service

Running `mouse-entropy start` manually will keep it in the foreground. For a persistent service, especially on an Arch-based system, you can use the included systemd unit file:

1. **Enable** and **start** the service:

   ```bash
   sudo systemctl enable mouse-entropy.service
   sudo systemctl start mouse-entropy.service
   ```

2. **Check logs**:

   ```bash
   journalctl -u mouse-entropy.service
   ```

3. **Stop** the service:

   ```bash
   sudo systemctl stop mouse-entropy.service
   ```

With systemd managing the process, it will automatically restart on reboots and run in the background.

---

## Data Storage Structure

When `mouse-entropy start` is running, it creates subdirectories under `base_directory` following this pattern:

```
base_directory/
└── YYYY
    └── MM
        └── DD
            ├── YYYY-MM-DD-00001.bin
            ├── YYYY-MM-DD-00002.bin
            └── ...
```

For each day, each chunk file is named:

```
YYYY-MM-DD-xxxxx.bin
```

Where `xxxxx` is a 5-digit zero-padded index (e.g., `00001`).

Each `.bin` file contains a raw sequence of 3 or 4 bytes per mouse event. The first byte is the button flags, and the second and third bytes are the signed `dx, dy` (movements). If `store_4_bytes` is `true`, a zero-padding byte is added as the fourth byte.

---

## Examples

### Quick Dump Example

Let's say you have been capturing data for a while and now want to convert a single day's data to CSV.

```bash
mouse-entropy dump --date 2025-01-01 --output january_1_data.csv
```

The resulting CSV file `january_1_data.csv` might look like this:

```csv
button,dx,dy
8,1,-1
8,1,0
8,1,2
8,1,1
...
```

- `button`: The raw button byte.
- `dx` and `dy`: Movement deltas (signed 8-bit integers).

### Clearing Data

To remove **all** collected data:

```bash
mouse-entropy clear
```

By default, it removes whatever is in `base_directory` (as configured by your TOML file). **Use with caution.**

### Viewing File Sizes

If you’d like to see how much data has been collected so far:

```bash
mouse-entropy size
```

Example output might be:

```
Year 2025 Month 01 Day 01 => 12345 bytes
Year 2025 Month 01 Day 02 => 45678 bytes
Year 2025 Month 01 => 58023 bytes
Year 2025 => 58023 bytes
Total across all years/months/days => 58023 bytes
```

---

## Logging & Debugging

This tool doesn’t provide extensive logging by default. However, if you run it in the foreground (i.e., `mouse-entropy start` directly), you can see:

- Basic console messages (starting, capturing, file rotation).
- Error messages if the device can’t be opened or read.

For systemd logging, use `journalctl -u mouse-entropy.service`.

---

## FAQ

1. **Why do I need root privileges to capture mouse events?**  
   By default, most Linux systems restrict read access to `/dev/input/mice`. You can change group permissions for the `input` group or run as root.

2. **How can I run this automatically on boot?**  
   Use the included systemd service file:

   ```bash
   sudo systemctl enable mouse-entropy.service
   sudo systemctl start mouse-entropy.service
   ```

3. **How do I parse the raw .bin files without dumping to CSV?**  
   Each record is 3 or 4 bytes. You can parse them in your own scripts if desired:

   - Byte 0: Button bits.
   - Byte 1: `dx` (signed).
   - Byte 2: `dy` (signed).
   - Byte 3 (optional): Zero padding if `store_4_bytes = true`.

4. **Can I store data somewhere other than `data/`?**  
   Yes, simply change the `base_directory` in `/etc/mouse-entropy/mouse-entropy.toml`.

5. **What about security concerns with storing raw mouse data?**
   - We store local files with restricted permissions (by default `0700` for directories, `0600` for files).
   - If you’re worried about the sensitivity of this data, ensure you have appropriate backups and protections.

---

## Troubleshooting

- **Permission Denied** when reading `/dev/input/mice`:  
  Make sure you run as `root` or have read permissions on that device.
- **No data is being generated**:  
  Check system logs (`journalctl -u mouse-entropy.service`) or run in the foreground to see errors. Perhaps your device path is different from `/dev/input/mice`.
- **Files are not rotating**:  
  Confirm `max_file_size_mb` in your config. Large mouse inactivity means it might take a while to reach the threshold.

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Make and test your changes.
4. Submit a Pull Request describing the changes.

Please open an issue first if you plan a significant change or need guidance.

---

## License

This project is licensed under the [MIT License](LICENSE).  
Feel free to use, modify, and distribute this software in accordance with the MIT license terms.

---

## Author / Maintainer

- **Author**: [Anthony Therrien](https://github.com/anto18671)

Contributions by the open-source community are greatly appreciated.

---

## Acknowledgments

- [Rust](https://www.rust-lang.org/) for making safe, performant CLIs easy.
- [Arch Linux](https://archlinux.org/) for providing a lightweight, DIY approach that fosters tools like this.

Feel free to leave a star on the [GitHub repository](https://github.com/anto18671/mouse-entropy) if you find this project useful!
