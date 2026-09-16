use std::env;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
        }

        Some("version") | Some("--version") | Some("-V") => {
            println!("Veyra {}", VERSION);
        }

        Some("install") => {
            match args.get(2) {
                Some(package) => {
                    println!("Veyra: installing '{}'", package);
                }
                None => {
                    println!("Veyra: missing package name.");
                    println!("Usage: veyra install <package>");
                }
            }
        }

        Some("remove") => {
            match args.get(2) {
                Some(package) => {
                    println!("Veyra: removing '{}'", package);
                }
                None => {
                    println!("Veyra: missing package name.");
                    println!("Usage: veyra remove <package>");
                }
            }
        }

        Some("search") => {
            match args.get(2) {
                Some(package) => {
                    println!("Veyra: searching for '{}'", package);
                }
                None => {
                    println!("Veyra: missing search query.");
                    println!("Usage: veyra search <package>");
                }
            }
        }

        Some("update") => {
            println!("Veyra: checking for system updates...");
        }

        Some("info") => {
            println!("Veyra Linux");
            println!("Version: {}", VERSION);
            println!("Base: Arch Linux");
        }

        Some("doctor") => {
            println!("Veyra Doctor");
            println!("System check started...");
        }

        Some(command) => {
            println!("Veyra: unknown command '{}'", command);
            println!("Run 'veyra help' for available commands.");
        }
    }
}
