use std::env;
use std::process::ExitCode;

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
            Some(query) => {
                if vpm_api::search(query) {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::from(1)
                }
            }
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

        Some("sync") => std::process::Command::new("sudo")
            .args(["pacman", "-Sy"])
            .status()
            .map(|status| ExitCode::from(status.code().unwrap_or(1) as u8))
            .unwrap_or_else(|_| ExitCode::from(1)),

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
