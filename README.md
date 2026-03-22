# tdgzi (Tar Dot Gz Installer)

![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust)
[![Stars](https://img.shields.io/github/stars/EnvizyWasTaken/tdgzi)](https://github.com/EnvizyWasTaken/tdgzi)
![Version](https://img.shields.io/badge/version-v0.2.0-blue)

> Install and uninstall .tar.gz apps like a package manager.

---

**tdgzi** is a lightweight CLI tool that simplifies installing software distributed as `.tar.gz` archives on Linux.

It analyzes archive contents, determines package type, installs it appropriately, and tracks installed files—bringing a **pacman-like experience to raw tarballs**.
---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Commands](#commands)
- [Examples](#examples)
- [How It Works](#how-it-works)
- [Install Tracking](#install-tracking)
- [Status](#status)
- [Safety Notice](#safety-notice)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- Automatic archive inspection (binary, source, script)
- Smart installation strategies based on package type
- Binary installs to `~/.local/bin`
- Source builds with proper `--prefix` handling
- Full install tracking (binary + source)
- Uninstall support
- Dry-run mode for safe previews
- Verbose mode for full build output
- Clean, minimal terminal UX (pacman-inspired)
- Written in Rust for speed and reliability

---

## Installation

```bash
git clone https://github.com/EnvizyWasTaken/tdgzi.git
cd tdgzi
cargo build --release
sudo cp target/release/tdgzi /usr/local/bin/
```

---

## Usage

```bash
tdgzi <COMMAND>
```

### Commands

| Command   | Description                 |
|-----------|-----------------------------|
| inspect   | Analyze a `.tar.gz` archive |
| install   | Install a `.tar.gz` archive |
| uninstall | Remove an installed package |
| help      | Show help information       |

---

## Examples

### Inspect an archive

```bash
tdgzi inspect package.tar.gz
```

Example output:

```text
[INFO] Files: 42
[INFO] Detected package type: Binary
[INFO] Executables:
 - project/bin/app
```

---

### Install a package

```bash
tdgzi install package.tar.gz
```

- Automatically detects package type
- Installs to `~/.local`
- Tracks installed files for removal

---

### Dry run

```bash
tdgzi install package.tar.gz --dry-run
```

Shows what would happen without making changes.

---

### Uninstall a package

```bash
tdgzi uninstall package-name
```

- Removes all tracked files
- Cleans up internal metadata

---

## How It Works

`tdgzi` scans archive contents and classifies them into:

- **Script-based** → contains install scripts (`install.sh`)
- **Source-based** → contains build systems (`Makefile`, `configure`, `CMakeLists.txt`)
- **Binary** → contains executable files
- **Unknown** → unclear structure

### Installation Strategies

- **Binary**  
  Extract → locate executable → copy to `~/.local/bin`

- **Source (GNU-style)**
  ```bash
  ./configure --prefix=$HOME/.local
  make
  make install DESTDIR=<staging>
  ```

- **Script**  
  Executes `install.sh` with confirmation

---

## Install Tracking

tdgzi tracks installed files per package:

```text
~/.local/share/tdgzi/db/
```

Each package has a JSON record:

```json
{
  "name": "example",
  "files": [
    "/home/user/.local/bin/example"
  ]
}
```

---

## Status

**v0.2.0 — Tracking + Uninstall Release**

### Supported

- Archive inspection
- Binary installation
- Source installation (GNU-style)
- Script installers (`install.sh`)
- Install tracking
- Uninstall

### Limitations

- No dependency resolution
- Some build systems may ignore `DESTDIR`
- No upgrade system yet
- Limited script detection

---

## Safety Notice

`tdgzi` automates installation, but does **not replace user judgment**.

Always:

- verify archive sources
- review scripts before executing them
- avoid installing untrusted software

---

## Roadmap

- Package listing (`tdgzi list`)
- Smarter script detection
- Better prefix enforcement
- Conflict detection
- Upgrade support
- Dependency hints

---

## Contributing

Contributions are welcome.

1. Fork the repository
2. Create a branch
3. Open a pull request

---

## License

MIT License

---

## Author

EnvizyWasTaken

---
