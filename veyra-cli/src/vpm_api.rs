use serde::Deserialize;
use std::process::Command;

const API_URL: &str = "https://veyra-api.abdualialderson.workers.dev";
const REPO_URL: &str =
    "https://raw.githubusercontent.com/abduali-sec/veyra-repo/main";

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    source: String,
}

pub fn repository() -> bool {
    let db_url = format!("{}/veyra.db.tar.gz", REPO_URL);

    println!("Veyra Repository");
    println!("Database: {}", db_url);

    match Command::new("curl")
        .args(["-fsSL", &db_url, "-o", "/tmp/veyra.db.tar.gz"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✓ Repository database downloaded.");
            true
        }

        _ => {
            eprintln!("✗ Failed to download repository database.");
            false
        }
    }
}

pub fn info(package: &str) -> bool {
    let url = format!("{}/packages/{}", API_URL, package);

    match Command::new("curl")
        .args(["-fsSL", &url])
        .output()
    {
        Ok(output) if output.status.success() => {
            match serde_json::from_slice::<Package>(&output.stdout) {
                Ok(pkg) => {
                    println!("Name: {}", pkg.name);
                    println!("Version: {}", pkg.version);
                    println!("Source: {}", pkg.source);
                    println!("Description: {}", pkg.description);
                    true
                }

                Err(error) => {
                    eprintln!("vpm: invalid API response: {}", error);
                    false
                }
            }
        }

        _ => {
            eprintln!("vpm: package '{}' not found", package);
            false
        }
    }
}

pub fn install(package: &str) -> bool {
    let file = format!("{}-0.1.0-1-x86_64.pkg.tar.zst", package);
    let url = format!("{}/{}", REPO_URL, file);
    let output = format!("/tmp/{}", file);

    println!("VPM → {}", url);

    match Command::new("curl")
        .args(["-fL", &url, "-o", &output])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✓ Package downloaded: {}", output);

            match Command::new("sudo")
                .args(["pacman", "-U", &output])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("✓ Package installed.");
                    true
                }

                _ => {
                    eprintln!("✗ Package installation failed.");
                    false
                }
            }
        }

        _ => {
            eprintln!("✗ Package download failed.");
            false
        }
    }
}
