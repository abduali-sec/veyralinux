use std::process::Command;

pub fn check() -> bool {
    println!("Veyra Doctor");
    println!();

    let pacman = check_command("pacman", &["--version"], "Pacman");
    let network = check_command("curl", &["--version"], "Network tools");
    let disk = check_command("df", &["-h", "/"], "Disk");
    let memory = check_command("free", &["-h"], "Memory");

    pacman && network && disk && memory
}

fn check_command(command: &str, args: &[&str], name: &str) -> bool {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            println!("✓ {}", name);
            true
        }

        _ => {
            println!("✗ {}", name);
            false
        }
    }
}
