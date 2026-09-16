use std::env;
use std::process::ExitCode;

mod doctor;
mod package_manager;
mod system;

const VERSION: &str = "0.1.0";

fn print_help() {
    println!(
        r#"
╔══════════════════════════════════════╗
║              VEYRA                   ║
║          Linux, reimagined           ║
╚══════════════════════════════════════╝

Usage:
    veyra <command>

Commands:
    install <package>   Install a package
    remove <package>    Remove a package
    search <package>    Search for a package
    update              Update the system
    info                Show system information
    doctor              Check system health
    help                Show this help
    version             Show Veyra version
"#
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
            ExitCode::SUCCESS
        }

        Some("version") | Some("--version") | Some("-V") => {
            println!("Veyra {}", VERSION);
            ExitCode::SUCCESS
        }

        Some("install") => match args.get(2) {
            Some(package) => {
                println!("Veyra: preparing to install '{}'", package);
                package_manager::install(package)
            }

            None => {
                eprintln!("Veyra: missing package name.");
                eprintln!("Usage: veyra install <package>");
                ExitCode::from(2)
            }
        },

        Some("remove") => match args.get(2) {
            Some(package) => {
                println!("Veyra: preparing to remove '{}'", package);
                package_manager::remove(package)
            }

            None => {
                eprintln!("Veyra: missing package name.");
                eprintln!("Usage: veyra remove <package>");
                ExitCode::from(2)
            }
        },

        Some("search") => match args.get(2) {
            Some(package) => package_manager::search(package),

            None => {
                eprintln!("Veyra: missing search query.");
                eprintln!("Usage: veyra search <package>");
                ExitCode::from(2)
            }
        },

        Some("update") => {
            println!("Veyra: updating system...");
            package_manager::update()
        }

        Some("info") => {
            system::info();
            ExitCode::SUCCESS
        }

        Some("doctor") => {
            if doctor::check() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }

        Some(command) => {
            eprintln!("Veyra: unknown command '{}'", command);
            eprintln!("Run 'veyra help' for available commands.");
            ExitCode::from(2)
        }
    }
}
