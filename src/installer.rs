use anyhow::{Result, anyhow};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;
use tempfile::tempdir;
use walkdir::WalkDir;

use std::os::unix::fs::PermissionsExt;

use crate::scan::ArchiveAnalysis;
use crate::rules::{classify, PackageType};

/// Entry point for installation
pub fn install(path: &str, analysis: &ArchiveAnalysis) -> Result<()> {
    let package_type = classify(analysis);

    match package_type {
        PackageType::Binary => install_binary(path),
        PackageType::Source => Err(anyhow!("Source packages not supported yet")),
        PackageType::Script => Err(anyhow!("Script installers not supported yet")),
        PackageType::Unknown => Err(anyhow!("Unknown package type")),
    }
}

/// Extract archive into a temporary directory
fn extract_archive(path: &str) -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    let extract_path = temp_dir.path().to_path_buf();

    let file = File::open(path)?;
    let decompressed = GzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    archive.unpack(&extract_path)?;

    // Prevent deletion (temporary for now)
    std::mem::forget(temp_dir);

    Ok(extract_path)
}

/// Find a likely executable binary inside extracted files
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

    // Prefer binaries inside /bin/
    if let Some(bin) = candidates.iter().find(|p| p.to_string_lossy().contains("/bin/")) {
        return Ok(bin.clone());
    }

    // Fallback: first executable
    Ok(candidates[0].clone())
}

/// Install binary into ~/.local/bin
fn install_binary(path: &str) -> Result<()> {
    println!("[INFO] Extracting archive...");

    let extract_path = extract_archive(path)?;

    println!("[INFO] Searching for executable...");

    let binary_path = find_binary(&extract_path)?;

    println!("[INFO] Found binary: {}", binary_path.display());

    let home = std::env::var("HOME")?;
    let target_dir = PathBuf::from(home).join(".local/bin");

    fs::create_dir_all(&target_dir)?;

    let file_name = binary_path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid binary name"))?;

    let target_path = target_dir.join(file_name);

    // Handle overwrite safely
    if target_path.exists() {
        println!("[WARN] {} already exists", target_path.display());
        println!("[INFO] Overwriting...");
    }

    fs::copy(&binary_path, &target_path)?;

    println!("[INFO] Installed to {}", target_path.display());

    Ok(())
}