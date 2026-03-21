use crate::scan::ArchiveAnalysis;

pub const INSTALLATION_SCRIPTS: &[&str] =
    &["install.sh", "Makefile", "CMakeLists.txt", "configure"];

#[derive(Debug)]
pub enum PACKAGE_TYPE {
    Binary,
    Source,
    Script,
    Unknown,
}

pub fn classify(analysis: &ArchiveAnalysis) -> PACKAGE_TYPE {
    let has_script = analysis.executables.iter().any(|file| {
        INSTALLATION_SCRIPTS.iter().any(|script| file.contains(script))
    });

    if has_script {
        PACKAGE_TYPE::Script
    } else if analysis.has_makefile {
        PACKAGE_TYPE::Source
    } else if !analysis.executables.is_empty() {
        PACKAGE_TYPE::Binary
    } else {
        PACKAGE_TYPE::Unknown
    }
}
