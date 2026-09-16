use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub version: String,
    pub package_backend: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            version: "0.1.0".to_string(),
            package_backend: "pacman".to_string(),
        }
    }
}

pub fn path() -> PathBuf {
    PathBuf::from("/etc/veyra.conf")
}

pub fn show() {
    let config = Config::load();

    println!("Veyra Configuration");
    println!("Config: {}", path().display());
    println!("Version: {}", config.version);
    println!("Package backend: {}", config.package_backend);

    if path().exists() {
        println!("Status: installed");
    } else {
        println!("Status: default");
    }

    let _ = fs::metadata(path());
}
