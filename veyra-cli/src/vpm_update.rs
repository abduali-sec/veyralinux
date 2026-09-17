use std::process::Command;

use crate::vpm_repo;

pub fn update() -> bool {
    let installed = match Command::new("pacman")
        .args(["-Q"])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).into_owned()
        }

        _ => {
            eprintln!("vpm: failed to read installed packages");
            return false;
        }
    };

    println!("VEYRA UPDATE");
    println!("=============");
    println!();

    let mut checked = 0;
    let mut updated = 0;

    for line in installed.lines() {
        let mut parts = line.split_whitespace();

        let name = match parts.next() {
            Some(value) if value.starts_with("veyra-") => value,
            _ => continue,
        };

        let installed_version = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        checked += 1;

        let repo_package = match vpm_repo::find_package(name) {
            Ok(Some(pkg)) => pkg,
            Ok(None) => {
                println!(
                    "  [WARN] {} {} is not in Veyra Repository",
                    name, installed_version
                );
                continue;
            }

            Err(error) => {
                eprintln!("  [FAIL] {}: {}", name, error);
                return false;
            }
        };

        let comparison = match Command::new("vercmp")
            .args([installed_version, &repo_package.version])
            .output()
        {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<i32>()
                    .unwrap_or(0)
            }

            _ => {
                eprintln!("vpm: vercmp is unavailable");
                return false;
            }
        };

        if comparison < 0 {
            println!(
                "  [UPDATE] {} {} -> {}",
                name,
                installed_version,
                repo_package.version
            );

            let url = format!(
                "https://raw.githubusercontent.com/abduali-sec/veyra-repo/main/{}",
                repo_package.filename
            );

            let output_path = format!(
                "/tmp/{}",
                repo_package.filename
            );

            let download = Command::new("curl")
                .args([
                    "-fL",
                    &url,
                    "-o",
                    &output_path,
                ])
                .status();

            match download {
                Ok(status) if status.success() => {}

                _ => {
                    eprintln!("  [FAIL] failed to download {}", name);
                    return false;
                }
            }

            let actual_sha256 = match Command::new("sha256sum")
                .arg(&output_path)
                .output()
            {
                Ok(output) if output.status.success() => {
                    String::from_utf8_lossy(&output.stdout)
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string()
                }

                _ => {
                    eprintln!("  [FAIL] failed to calculate SHA-256 for {}", name);
                    return false;
                }
            };

            if actual_sha256 != repo_package.sha256 {
                eprintln!("  [FAIL] SHA-256 verification failed for {}", name);
                return false;
            }

            println!("  [OK] SHA-256 verified");

            match Command::new("sudo")
                .args(["pacman", "-U", &output_path])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("  [OK] {} updated", name);
                    updated += 1;
                }

                _ => {
                    eprintln!("  [FAIL] failed to install {}", name);
                    return false;
                }
            }
        } else {
            println!(
                "  [OK] {} {} is current",
                name,
                installed_version
            );
        }
    }

    println!();

    if checked == 0 {
        println!("No installed Veyra packages found.");
    } else if updated == 0 {
        println!("All installed Veyra packages are up to date.");
    } else {
        println!("Updated packages: {}", updated);
    }

    true
}
