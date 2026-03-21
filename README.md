# tdgzi (Tar Dot Gz Installer)

**tdgzi** is a lightweight CLI tool that simplifies installing software distributed as `.tar.gz` archives on Linux.

It inspects archive contents, determines what kind of package you’re dealing with, and helps you install it without the usual guesswork.

---

## Features

- Fast, minimal workflow for `.tar.gz` installs
- Automatic archive inspection (scripts, source builds, binaries)
- Basic binary installation support (`~/.local/bin`)
- Reduces manual steps and guesswork
- Written in Rust for speed and reliability

---

## Installation

Build from source:

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

| Command  | Description                              |
|----------|------------------------------------------|
| inspect  | Analyze a `.tar.gz` archive              |
| install  | Install supported `.tar.gz` archives     |
| help     | Show help information                    |

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

### Install a binary package

```bash
tdgzi install package.tar.gz
```

Installs detected binaries to:

```text
~/.local/bin
```

---

## How It Works

`tdgzi` scans archive contents and classifies them into:

- **Script-based** → contains install scripts (`install.sh`, `configure`)
- **Source-based** → contains build systems (`Makefile`, `CMakeLists.txt`)
- **Binary** → contains executable files
- **Unknown** → unclear structure

This classification determines how installation is handled.

---

## Status

**v0.1.0 — Initial release**

### Currently supported
- Archive inspection
- Binary package installation

### Not yet supported
- Script-based installers (`install.sh`, `configure`)
- Source builds (`Makefile`, `CMake`)
- Install tracking / uninstall

---

## Why tdgzi?

Installing from `.tar.gz` archives usually means:

- extracting files
- digging through directories
- guessing the right command
- hoping it works

`tdgzi` makes that process predictable.

---

## Philosophy

- Keep it simple
- Be predictable
- Don’t break the user’s system
- Automate the boring parts, not the dangerous ones

---

## Safety Notice

`tdgzi` automates parts of installation, but it does **not replace user judgment**.

Always:
- verify archive sources
- review scripts before running them
- avoid installing untrusted software

---

## Roadmap

- Smarter binary detection
- Dry-run mode
- Safer overwrite handling
- Script execution support
- Install tracking and uninstall

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

Envizy

---

**tdgzi — because manually installing tarballs is a personality trait you don’t need.**