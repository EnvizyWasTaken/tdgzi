use std::{fs, path::PathBuf};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub files: Vec<String>,
}

fn db_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    PathBuf::from(home).join(".local/share/tdgzi/db")
}

pub fn save(pkg: &Package) -> Result<()> {
    let dir = db_dir();
    fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{}.json", pkg.name));
    let data = serde_json::to_string_pretty(pkg)?;

    fs::write(path, data)?;
    Ok(())
}


pub fn load(name: &str) -> Result<Package> {
    let path = db_dir().join(format!("{}.json", name));
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn remove(name: &str) -> Result<()> {
    let path = db_dir().join(format!("{}.json", name));
    fs::remove_file(path)?;
    Ok(())
}

