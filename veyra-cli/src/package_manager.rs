use std::process::{Command, ExitCode};

#[allow(dead_code)]
pub fn search(package: &str) -> ExitCode {
    run_pacman(&["-Ss", package], false)
}

pub fn install(package: &str) -> ExitCode {
    run_pacman(&["-S", package], true)
}

pub fn remove(package: &str) -> ExitCode {
    run_pacman(&["-R", package], true)
}

pub fn update() -> ExitCode {
    run_pacman(&["-Syu"], true)
}

fn run_pacman(args: &[&str], use_sudo: bool) -> ExitCode {
    let display = format!("pacman {}", args.join(" "));
    println!("Veyra → {}", display);

    let result = if use_sudo {
        Command::new("sudo").arg("pacman").args(args).status()
    } else {
        Command::new("pacman").args(args).status()
    };

    match result {
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
