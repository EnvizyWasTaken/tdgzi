use crate::scan::ArchiveAnalysis;

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
    let has_script = analysis.executables.iter().any(|file| {
        INSTALLATION_SCRIPTS.iter().any(|script| file.contains(script))
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
