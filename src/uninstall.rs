use anyhow::Result;
use std::fs;
use std::io::{self, Write};
use console::style;

use crate::db;

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

pub fn run(name: &str) -> Result<()> {
    step(&format!("Uninstalling {}", name));

    let pkg = db::load(name)?;

    substep(&format!("{} file(s) removed", pkg.files.len()));

    if !confirm("Proceed with uninstall?", false)? {
        println!("Aborted.");
        return Ok(());
    }

    println!();

    for file in &pkg.files {
        substep(&format!("Removing {}", file));

        if let Err(e) = fs::remove_file(file) {
            eprintln!("    -> Failed: {}", e);
        }
    }

    db::remove(name)?;

    step(&format!("Changes to {} written", name));

    println!();
    success("Uninstalled successfully");

    Ok(())
}