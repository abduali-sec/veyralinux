use serde::Deserialize;
use std::process::Command;

const API_URL: &str = "https://veyra-api.abdualialderson.workers.dev";
const REPO_URL: &str = "https://raw.githubusercontent.com/abduali-sec/veyra-repo/main";

#[derive(Deserialize)]
struct SearchResponse {
    results: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    source: String,
}

pub fn search(query: &str) -> bool {
    let url = format!("{}/search?q={}", API_URL, query);

    match Command::new("curl")
        .args(["-fsSL", &url])
        .output()
    {
        Ok(output) if output.status.success() => {
            match serde_json::from_slice::<SearchResponse>(&output.stdout) {
                Ok(response) => {
                    println!();
                    println!("VEYRA PACKAGES");
                    println!("==============");

                    if response.results.is_empty() {
                        println!("No packages found.");
                        return true;
                    }

                    for pkg in response.results {
                        println!(
                            "{:<22} {:<10} [{}]\n    {}",
                            pkg.name,
                            pkg.version,
                            pkg.source,
                            pkg.description
                        );
                    }

                    true
                }

                Err(error) => {
                    eprintln!("vpm: invalid API response: {}", error);
                    false
                }
            }
        }

        _ => {
            eprintln!("vpm: API request failed");
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
    let url = format!("{}/packages/{}", API_URL, package);

    println!("VPM → {}", url);

    match Command::new("curl")
        .args(["-fsSL", &url])
        .output()
    {
        Ok(output) if output.status.success() => {
            match serde_json::from_slice::<Package>(&output.stdout) {
                Ok(pkg) => {
                    println!();
                    println!("VEYRA PACKAGE");
                    println!("=============");
                    println!("Name: {}", pkg.name);
                    println!("Version: {}", pkg.version);
                    println!("Source: {}", pkg.source);
                    println!("Description: {}", pkg.description);
                    println!();
                    println!("Package download/install pipeline is not implemented yet.");
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

        Ok(_) => {
            eprintln!("✗ Failed to download repository database.");
            false
        }

        Err(error) => {
            eprintln!("✗ Failed to start downloader: {}", error);
            false
        }
    }
}
