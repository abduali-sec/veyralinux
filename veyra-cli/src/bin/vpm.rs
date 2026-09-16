use std::env;
use std::process::ExitCode;

mod package_manager;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2) {
            Some(package) => package_manager::install(package),
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
            Some(package) => package_manager::search(package),
            None => {
                eprintln!("vpm: missing search query");
                ExitCode::from(2)
            }
        },

        Some("update") => package_manager::update(),

        Some("sync") => {
            println!("vpm: syncing package databases...");
            std::process::Command::new("sudo")
                .args(["pacman", "-Sy"])
                .status()
                .map(|status| ExitCode::from(status.code().unwrap_or(1) as u8))
                .unwrap_or_else(|_| ExitCode::from(1))
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
            println!("  vpm update");
            println!("  vpm sync");
            println!("  vpm version");
            ExitCode::SUCCESS
        }
    }
}
