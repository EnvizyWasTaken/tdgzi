# tdgzi (Tar Dot Gz Installer)

**tdgzi** is a minimal Linux utility designed to simplify and standardize the installation of `.tar.gz` archives. It automates common steps to help you install software faster, reduce manual errors, and keep your system clean.

---

## ✨ Features

* ⚡ **Fast installation workflow** for `.tar.gz` packages
* 🧠 **Automatic inspection** of archive contents
* 🧹 Helps prevent messy or inconsistent installs
* 🪶 Lightweight and minimal by design
* 🛠 Built for developers and power users who work with source archives frequently

---

## 📦 Installation

> *Add your installation steps here (e.g., build from source, package manager, etc.)*

Example:

```bash
git clone https://github.com/yourusername/tdgzi.git
cd tdgzi
make
sudo make install
```

---

## 🚀 Usage

```bash
tdgzi <COMMAND>
```

### Available Commands

| Command | Description                                   |
| ------- | --------------------------------------------- |
| inspect | Analyze a `.tar.gz` archive before installing |
| help    | Show help information for commands            |

### Options

| Option     | Description        |
| ---------- | ------------------ |
| -h, --help | Print help message |

---

## 🔍 Examples

### Inspect an archive

```bash
tdgzi inspect package.tar.gz
```

This will analyze the archive structure and help determine how it should be installed.

---

## 🎯 Purpose

Installing software from `.tar.gz` archives often involves repetitive steps:

* Extracting files
* Reading documentation
* Running build/install scripts
* Deciding install locations

**tdgzi** streamlines this process to:

* Save time
* Reduce mistakes
* Provide a consistent installation experience

---

## 🧩 Philosophy

* Keep it simple
* Do one thing well
* Avoid unnecessary complexity
* Respect the user's system

---

## ⚠️ Disclaimer

`tdgzi` aims to assist with installations, but users should still review software sources and installation steps when necessary. Always verify the integrity and safety of archives before installing.

---

## 🤝 Contributing

Contributions, suggestions, and improvements are welcome!

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

---

## 📄 License

> *I use the MIT license, if you wanna know what it is js search it up gng :broken_heart:*

---

## 💡 Future Ideas

* Install location suggestions
* Rollback support
* Package tracking

---

## 🧑‍💻 Author

> *Yours Truly: Envizy*

---

**tdgzi — install smarter, not harder.**
