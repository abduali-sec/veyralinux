use serde::Deserialize;
use std::process::Command;

const API_URL: &str = "https://veyra-api.abdualialderson.workers.dev";

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
