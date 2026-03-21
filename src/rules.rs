use crate::scan::ArchiveAnalysis;
use std::path::Path;

pub const INSTALLATION_SCRIPTS: &[&str] =
    &["install.sh", "Makefile", "CMakeLists.txt", "configure"];

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
            INSTALLATION_SCRIPTS.iter().any(|script| name == *script)
        } else {
            false
        }
    });

    if has_script {
        PackageType::Script
    } else if analysis.has_makefile {
        PackageType::Source
    } else if !analysis.executables.is_empty() {
        PackageType::Binary
    } else {
        PackageType::Unknown
    }
}