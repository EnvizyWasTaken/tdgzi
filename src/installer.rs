use anyhow::{Result, anyhow};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Write, Read};

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
pub fn install(path: &str, analysis: &ArchiveAnalysis, dry_run: bool) -> Result<()> {
    let package_type = classify(analysis);

    match package_type {
        PackageType::Binary => install_binary(path, dry_run),
        PackageType::Source => Err(anyhow!("Source packages not supported yet")),
        PackageType::Script => Err(anyhow!("Script installers not supported yet")),
        PackageType::Unknown => Err(anyhow!("Unknown package type")),
    }
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

fn extract_archive(path: &str) -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    let extract_path = temp_dir.path().to_path_buf();

    let file = File::open(path)?;
    let decompressed = GzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    archive.unpack(&extract_path)?;

    std::mem::forget(temp_dir); // keep temp dir

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

    // Skip progress bar for small files
    if total_size < 100_000 {
        std::io::copy(&mut input, &mut output)?;
        return Ok(());
    }

    let pb = ProgressBar::new(total_size);

    pb.set_style(
        ProgressStyle::with_template(
            "[{bar:40}] {bytes}/{total_bytes} ({percent}%)"
        )
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

    pb.finish(); // keep visible

    Ok(())
}

fn install_binary(path: &str, dry_run: bool) -> Result<()> {
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