use anyhow::Result;
use flate2::read::GzDecoder;
use std::path::Path;
use std::fs::{File, FileType};
use tar::Archive;

#[derive(Debug)]
pub struct ArchiveAnalysis {
    pub file_count: usize,
    pub has_makefile: bool,
    pub executables: Vec<String>,
    pub files: Vec<String>,
}

pub fn analyze_archive(path: &str) -> Result<ArchiveAnalysis> {
    let file = File::open(path)?;
    let decompressed = GzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    let mut file_count = 0;
    let mut has_makefile = false;
    let mut executables = Vec::new();

    let mut files = Vec::new();

    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        let path_str = path.to_string_lossy().to_string();

        file_count += 1;
        files.push(path_str.clone());

        if let Some(name) = Path::new(&path_str).file_name() {
            if name == "Makefile" {
                has_makefile = true;
            }
        }

        if let Ok(mode) = entry.header().mode() {
            if mode & 0o111 != 0 {
                executables.push(path_str);
            }
        }
    }

    Ok(ArchiveAnalysis {
        file_count,
        has_makefile,
        executables,
        files,
    })
}
