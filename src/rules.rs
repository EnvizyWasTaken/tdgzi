use crate::scan::ArchiveAnalysis;
use std::path::Path;

pub const SCRIPT_FILES: &[&str] = &["install.sh", "configure"];
pub const BUILD_FILES: &[&str] = &["Makefile", "CMakeLists.txt"];

#[derive(Debug)]
pub enum PackageType {
    Binary,
    Source,
    Script,
    Unknown,
}

pub fn classify(analysis: &ArchiveAnalysis) -> PackageType {
    let has_script = analysis.files.iter().any(|file| {
        if let Some(name) = Path::new(file).file_name() {
            SCRIPT_FILES.iter().any(|script| name == *script)
        } else {
            false
        }
    });

    let has_build = analysis.files.iter().any(|file| {
        if let Some(name) = Path::new(file).file_name() {
            BUILD_FILES.iter().any(|build| name == *build)
        } else {
            false
        }
    });

    if has_script {
        PackageType::Script
    } else if has_build {
        PackageType::Source
    } else if !analysis.executables.is_empty() {
        PackageType::Binary
    } else {
        PackageType::Unknown
    }
}