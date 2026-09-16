use std::process::Command;

pub fn sync() -> bool {
    println!("Veyra Repository");
    println!("Synchronizing package databases...");

    match Command::new("sudo").arg("pacman").args(["-Sy"]).status() {
        Ok(status) if status.success() => {
            println!("✓ Package databases synchronized.");
            true
        }

        Ok(_) => {
            eprintln!("✗ Failed to synchronize package databases.");
            false
        }

        Err(error) => {
            eprintln!("✗ Failed to start pacman: {}", error);
            false
        }
    }
}
