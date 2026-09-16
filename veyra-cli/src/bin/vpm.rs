use std::env;
use std::process::ExitCode;

mod vpm_repo {
    include!("../vpm_repo.rs");
}

mod vpm_api {
    include!("../vpm_api.rs");
}

mod package_manager {
    include!("../package_manager.rs");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2) {
            Some(package) => {
                if vpm_api::install(package) {
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

        Some("remove") => match args.get(2) {
            Some(package) => package_manager::remove(package),
            None => {
                eprintln!("vpm: missing package name");
                ExitCode::from(2)
            }
        },

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
                            println!("{:<24} {}\n    {}", pkg.name, pkg.version, pkg.description);
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

        Some("update") => package_manager::update(),

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
