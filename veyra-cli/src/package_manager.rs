use std::process::{Command, ExitCode};

pub fn run(args: &[&str]) -> ExitCode {
    println!("Veyra → pacman {}", args.join(" "));

    match Command::new("sudo").arg("pacman").args(args).status() {
        Ok(status) => match status.code() {
            Some(code) => ExitCode::from(code as u8),
            None => {
                eprintln!("Veyra: pacman terminated unexpectedly.");
                ExitCode::from(1)
            }
        },

        Err(error) => {
            eprintln!("Veyra: failed to start pacman: {}", error);
            ExitCode::from(1)
        }
    }
}
