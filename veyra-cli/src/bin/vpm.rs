use std::env;
use std::process::{Command, ExitCode};

mod vpm_api {
    include!("../vpm_api.rs");
}

mod vpm_repo {
    include!("../vpm_repo.rs");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2) {
            Some(package) => match vpm_repo::find_package(package) {
                Ok(Some(pkg)) => {
                    let url = format!(
                        "https://raw.githubusercontent.com/abduali-sec/veyra-repo/main/{}",
                        pkg.filename
                    );

                    let output = format!("/tmp/{}", pkg.filename);

                    println!("VPM → {}", url);

                    let downloaded = Command::new("curl")
                        .args(["-fL", &url, "-o", &output])
                        .status()
                        .map(|status| status.success())
                        .unwrap_or(false);

                    if !downloaded {
                        eprintln!("✗ Package download failed.");
                        return ExitCode::from(1);
                    }

                    println!("✓ Package downloaded.");

                    let actual = match Command::new("sha256sum").arg(&output).output() {
                        Ok(output) if output.status.success() => {
                            String::from_utf8_lossy(&output.stdout)
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string()
                        }

                        _ => {
                            eprintln!("✗ Failed to calculate SHA256.");
                            return ExitCode::from(1);
                        }
                    };

                    println!("SHA256 expected: {}", pkg.sha256);
                    println!("SHA256 actual:   {}", actual);

                    if pkg.sha256.is_empty() || actual != pkg.sha256 {
                        eprintln!("✗ SHA256 verification failed.");
                        return ExitCode::from(1);
                    }

                    println!("✓ SHA256 verified.");

                    let installed = Command::new("sudo")
                        .args(["pacman", "-U", &output])
                        .status()
                        .map(|status| status.success())
                        .unwrap_or(false);

                    if installed {
                        println!("✓ Package installed.");
                        ExitCode::SUCCESS
                    } else {
                        eprintln!("✗ Package installation failed.");
                        ExitCode::from(1)
                    }
                }

                Ok(None) => {
                    eprintln!("vpm: package '{}' not found", package);
                    ExitCode::from(1)
                }

                Err(error) => {
                    eprintln!("vpm: {}", error);
                    ExitCode::from(1)
                }
            },

            None => {
                eprintln!("vpm: missing package name");
                ExitCode::from(2)
            }
        },

        Some("remove") => {
            eprintln!("vpm: remove is not implemented in the Veyra backend yet.");
            ExitCode::from(1)
        }

        Some("search") => match args.get(2) {
            Some(query) => match vpm_repo::search(query) {
                Ok(results) => {
                    println!();
                    println!("VEYRA REPOSITORY");
                    println!("================");

                    if results.is_empty() {
                        println!("No packages found.");
                    } else {
                        for pkg in results {
                            println!("{:<24} {}", pkg.name, pkg.version);
                            println!("    {}", pkg.description);
                        }
                    }

                    ExitCode::SUCCESS
                }

                Err(error) => {
                    eprintln!("vpm: {}", error);
                    ExitCode::from(1)
                }
            },

            None => {
                eprintln!("vpm: missing search query");
                ExitCode::from(2)
            }
        },

        Some("info") => match args.get(2) {
            Some(package) => {
                if vpm_api::info(package) {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::from(1)
                }
            }

            None => {
                eprintln!("vpm: missing package name");
                ExitCode::from(2)
            }
        },

        Some("update") => {
            if vpm_api::repository() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }

        Some("sync") => {
            if vpm_api::repository() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }

        Some("version") | Some("--version") | Some("-V") => {
            println!("vpm 0.1.0");
            ExitCode::SUCCESS
        }

        _ => {
            println!("vpm - Veyra Package Manager");
            println!();
            println!("Usage:");
            println!("  vpm install <package>");
            println!("  vpm remove <package>");
            println!("  vpm search <package>");
            println!("  vpm info <package>");
            println!("  vpm update");
            println!("  vpm sync");
            println!("  vpm version");

            ExitCode::SUCCESS
        }
    }
}
