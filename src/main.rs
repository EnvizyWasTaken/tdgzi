mod installer;
mod rules;
mod scan;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tdgzi",
    about = "A smart installer for .tar.gz archives",
    long_about = "tdgzi analyzes .tar.gz archives, detects their structure, and helps install them safely and consistently.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Analyze a .tar.gz archive",
        long_about = "Inspects the archive contents and determines whether it is a binary, source package, or script-based installer."
    )]
    Inspect {
        #[arg(help = "Path to the .tar.gz file")]
        file: String,
    },

    #[command(
        about = "Install a .tar.gz archive",
        long_about = "Extracts the archive, detects its type, and installs it (currently supports binary packages into ~/.local/bin)."
    )]
    Install {
        #[arg(help = "Path to the .tar.gz file")]
        file: String,
    },
}

fn analyze_and_classify(
    file: &str,
) -> anyhow::Result<(scan::ArchiveAnalysis, rules::PackageType)> {
    let analysis = scan::analyze_archive(file)?;
    let package = rules::classify(&analysis);
    Ok((analysis, package))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { file } => {
            match analyze_and_classify(&file) {
                Ok((analysis, package)) => {
                    println!("[INFO] Files: {}", analysis.file_count);
                    println!("[INFO] Detected package type: {:?}", package);

                    println!("[INFO] Executables:");
                    for exe in &analysis.executables {
                        println!(" - {}", exe);
                    }

                    if analysis.executables.is_empty() {
                        println!("[WARN] No executables found");
                    }
                }
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }

        Commands::Install { file } => {
            match analyze_and_classify(&file) {
                Ok((analysis, package)) => {
                    println!("[INFO] Detected package type: {:?}", package);

                    if let Err(e) = installer::install(&file, &analysis) {
                        eprintln!("[ERROR] {}", e);
                    } else {
                        println!("[INFO] Installation complete ({:?})", package);
                    }
                }
                Err(e) => eprintln!("[ERROR] {}", e),
            }
        }
    }
}