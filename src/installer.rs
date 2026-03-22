use anyhow::{Result, anyhow};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Write, Read};
use std::process::{Command, Stdio};

use flate2::read::GzDecoder;
use tar::Archive;
use tempfile::tempdir;
use walkdir::WalkDir;

use indicatif::{ProgressBar, ProgressStyle};
use console::style;

use std::os::unix::fs::PermissionsExt;

use crate::scan::ArchiveAnalysis;
use crate::rules::{classify, PackageType};

/// Entry point
pub fn install(
    path: &str,
    analysis: &ArchiveAnalysis,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    let package_type = classify(analysis);

    match package_type {
        PackageType::Binary => install_binary(path, dry_run, verbose),
        PackageType::Source => install_source(path, dry_run, verbose),
        PackageType::Script => install_script(path, dry_run, verbose),
        PackageType::Unknown => Err(anyhow!("Unknown package type")),
    }
}

fn run(cmd: &mut Command, verbose: bool) -> Result<()> {
    if !verbose {
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null());
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(anyhow!("Command failed"));
    }

    Ok(())
}

fn confirm(prompt: &str, default: bool) -> Result<bool> {
    let suffix = if default { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", prompt, suffix);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Ok(default);
    }

    Ok(input == "y" || input == "yes")
}

fn step(message: &str) {
    println!("{}", style(format!("==> {}", message)).bold());
}

fn substep(message: &str) {
    println!("{}", style(format!("    -> {}", message)).dim());
}

fn success(message: &str) {
    println!("{}", style(format!(":: {}", message)).green().bold());
}

fn fake_progress(msg: &str) {
    use std::{thread, time::Duration};

    let pb = ProgressBar::new(100);

    pb.set_style(
        ProgressStyle::with_template("[{bar:30}] {percent}%")
            .unwrap()
            .progress_chars("=> ")
    );

    pb.set_message(msg.to_string());

    for i in 0..100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(8));
    }

    pb.finish_and_clear();
}

fn extract_archive(path: &str) -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    let extract_path = temp_dir.path().to_path_buf();

    let file = File::open(path)?;
    let decompressed = GzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    archive.unpack(&extract_path)?;

    std::mem::forget(temp_dir);

    Ok(extract_path)
}

fn find_binary(extract_path: &Path) -> Result<PathBuf> {
    let mut candidates = Vec::new();

    for entry in WalkDir::new(extract_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(path)?;

            if metadata.permissions().mode() & 0o111 != 0 {
                candidates.push(path.to_path_buf());
            }
        }
    }

    if candidates.is_empty() {
        return Err(anyhow!("No executable found"));
    }

    if let Some(bin) = candidates.iter().find(|p| p.to_string_lossy().contains("/bin/")) {
        return Ok(bin.clone());
    }

    Ok(candidates[0].clone())
}

fn copy_with_progress(src: &Path, dst: &Path) -> Result<()> {
    let mut input = File::open(src)?;
    let mut output = File::create(dst)?;

    let total_size = input.metadata()?.len();

    if total_size < 100_000 {
        std::io::copy(&mut input, &mut output)?;
        return Ok(());
    }

    let pb = ProgressBar::new(total_size);

    pb.set_style(
        ProgressStyle::with_template("[{bar:40}] {bytes}/{total_bytes} ({percent}%)")
            .unwrap()
            .progress_chars("█░ ")
    );

    let mut buffer = [0u8; 8192];
    let mut copied = 0;

    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        output.write_all(&buffer[..n])?;
        copied += n as u64;
        pb.set_position(copied);
    }

    pb.finish_with_message("done");

    Ok(())
}

fn install_binary(path: &str, dry_run: bool, _verbose: bool) -> Result<()> {
    step("Extracting archive...");
    let extract_path = extract_archive(path)?;

    step("Searching for executable...");
    let binary_path = find_binary(&extract_path)?;
    substep(&format!("Using {}", binary_path.display()));

    let home = std::env::var("HOME")?;
    let target_dir = PathBuf::from(home).join(".local/bin");

    let file_name = binary_path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid binary name"))?;

    let target_path = target_dir.join(file_name);

    if dry_run {
        step("Dry run summary");
        substep(&format!("Would create {}", target_dir.display()));
        substep(&format!("Would install {}", target_path.display()));
        return Ok(());
    }

    fs::create_dir_all(&target_dir)?;

    if target_path.exists() {
        substep(&format!("{} already exists", target_path.display()));

        if !confirm("Overwrite?", false)? {
            return Err(anyhow!("Installation aborted by user"));
        }
    }

    step("Installing binary...");
    copy_with_progress(&binary_path, &target_path)?;

    substep(&format!("Installed to {}", target_path.display()));

    println!();
    success("Installation complete");

    Ok(())
}

fn install_source(path: &str, dry_run: bool, verbose: bool) -> Result<()> {
    step("Extracting archive...");
    let extract_path = extract_archive(path)?;

    let project_dir = fs::read_dir(&extract_path)?
        .filter_map(|e| e.ok())
        .find(|e| e.path().is_dir())
        .map(|e| e.path())
        .ok_or_else(|| anyhow!("Could not find project directory"))?;

    substep(&format!("Project root: {}", project_dir.display()));

    let home = std::env::var("HOME")?;
    let prefix = format!("{}/.local", home);
    substep(&format!("Using prefix {}", prefix));

    if dry_run {
        step("Dry run summary");
        substep("Would run: ./configure --prefix=$HOME/.local");
        substep("Would run: make");
        substep("Would run: make install");
        return Ok(());
    }

    println!();

    if project_dir.join("configure").exists() {
        step("Configuring...");
        if !verbose {
            fake_progress("configure");
        }

        run(
            &mut Command::new("./configure")
                .arg(format!("--prefix={}", prefix))
                .current_dir(&project_dir),
            verbose,
        )?;
    }

    println!();

    step("Building...");
    if !verbose {
        fake_progress("make");
    }

    run(
        &mut Command::new("make")
            .current_dir(&project_dir),
        verbose,
    )?;

    println!();

    step("Installing...");
    if !verbose {
        fake_progress("install");
    }

    run(
        &mut Command::new("make")
            .arg("install")
            .current_dir(&project_dir),
        verbose,
    )?;

    println!();
    success("Installation complete");

    Ok(())
}

fn install_script(path: &str, dry_run: bool, verbose: bool) -> Result<()> {
    step("Extracting archive...");
    let extract_path = extract_archive(path)?;

    let script = WalkDir::new(&extract_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path()
                .file_name()
                .map(|n| n == "install.sh")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .ok_or_else(|| anyhow!("No install.sh found"))?;

    substep(&format!("Found script: {}", script.display()));

    if dry_run {
        step("Dry run summary");
        substep(&format!("Would execute {}", script.display()));
        return Ok(());
    }

    println!("{}", style(":: Warning: executing install script").yellow());

    if !confirm("Execute install script?", false)? {
        return Err(anyhow!("Installation aborted by user"));
    }

    step("Running install script...");

    run(
        &mut Command::new("sh")
            .arg(&script)
            .current_dir(&extract_path),
        verbose,
    )?;

    println!();
    success("Script installation complete");

    Ok(())
}