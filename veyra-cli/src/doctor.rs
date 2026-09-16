use std::process::Command;

pub fn check() {
    println!("Veyra Doctor");
    println!();

    check_command("pacman", &["--version"], "Pacman");
    check_command("curl", &["--version"], "Network tools");
    check_command("df", &["-h", "/"], "Disk");
    check_command("free", &["-h"], "Memory");
}

fn check_command(command: &str, args: &[&str], name: &str) {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            println!("✓ {}", name);
        }

        _ => {
            println!("✗ {}", name);
        }
    }
}
