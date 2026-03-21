# tdgzi (Tar Dot Gz Installer)

**tdgzi** is a lightweight CLI tool that simplifies installing software distributed as `.tar.gz` archives on Linux.

It inspects archive contents, figures out what kind of package you’re dealing with, and helps you install it without the usual “what am I supposed to do with this?”.

---

## Features

- Fast, minimal workflow for `.tar.gz` installs  
- Automatic archive inspection (scripts, source builds, binaries)  
- Reduces manual steps and guesswork  
- Designed to stay out of your way  
- Written in Rust for speed and reliability  

---

## Installation

Build from source:

```bash
git clone https://github.com/yourusername/tdgzi.git
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

| Command  | Description                 |
|----------|-----------------------------|
| inspect  | Analyze a `.tar.gz` archive |
| help     | Install a `.tar.gz` archive |
| help     | Show help information       |

---

## Example

```bash
tdgzi inspect package.tar.gz
```

Example output:

```text
[INFO] Files: 42
[INFO] Executables:
 - project/bin/app
[INFO] Type: Binary
```

---

## How It Works

`tdgzi` scans archive contents and classifies them into one of the following:

- **Script-based** → contains install scripts (`install.sh`, `configure`)  
- **Source-based** → contains build systems (`Makefile`, `CMakeLists.txt`)  
- **Binary** → contains executable files  
- **Unknown** → unclear structure, manual intervention likely  

This classification is used to determine how installation should be handled.

---

## Why tdgzi?

Installing from `.tar.gz` archives usually looks like:

- extract files  
- poke around directories  
- read half a README  
- guess what command to run  
- hope it works  

`tdgzi` turns that into something predictable.

---

## Philosophy

- Keep it simple  
- Be predictable  
- Don’t break the user’s system  
- Automate the boring parts, not the dangerous ones  

---

## Safety Notice

This tool can automate parts of installation, but it does not replace common sense.

Always:
- verify archive sources  
- review scripts before running them  
- avoid installing untrusted software  

---

## Status

Early development. Core inspection and classification are implemented.

Planned next:
- Binary installation support  
- Script execution safeguards  
- Install tracking and uninstall  

---

## Roadmap

- Install tracking  
- Rollback support  
- Dry-run mode  
- Smarter detection heuristics  

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
