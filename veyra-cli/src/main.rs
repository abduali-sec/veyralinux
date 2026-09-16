use std::env;
use std::process::ExitCode;

mod cli;
mod config;
mod doctor;
mod downloader;
mod package_manager;
mod repository;
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
    config              Show Veyra configuration
    help                Show this help
    version             Show Veyra version
"#
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let command = match cli::parse(&args) {
        Ok(command) => command,

        Err(error) => {
            eprintln!("Veyra: {}", error);
            eprintln!("Run 'veyra help' for available commands.");
            return ExitCode::from(2);
        }
    };

    match command {
        cli::Command::Help => {
            print_help();
            ExitCode::SUCCESS
        }

        cli::Command::Version => {
            println!("Veyra {}", VERSION);
            ExitCode::SUCCESS
        }

        cli::Command::Install(package) => {
            println!("Veyra: preparing to install '{}'", package);
            package_manager::install(&package)
        }

        cli::Command::Remove(package) => {
            println!("Veyra: preparing to remove '{}'", package);
            package_manager::remove(&package)
        }

        cli::Command::Search(package) => package_manager::search(&package),

        cli::Command::Update => {
            println!("Veyra: updating system...");
            package_manager::update()
        }

        cli::Command::Info => {
            system::info();
            ExitCode::SUCCESS
        }

        cli::Command::Doctor => {
            if doctor::check() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }

        cli::Command::Config => {
            config::show();
            ExitCode::SUCCESS
        }

        cli::Command::Sync => {
            if repository::sync() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }

        cli::Command::Download(url, output) => {
            if downloader::download(&url, &output) {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}
