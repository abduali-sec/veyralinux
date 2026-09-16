use std::process::Command;

pub fn info() {
    println!("Veyra Linux");
    println!("Veyra version: 0.1.0");

    let kernel = command_output("uname", &["-r"]);
    println!("Kernel: {}", kernel);

    let arch = command_output("uname", &["-m"]);
    println!("Architecture: {}", arch);

    let hostname = command_output("hostname", &[]);
    println!("Hostname: {}", hostname);

    let uptime = command_output("uptime", &["-p"]);
    println!("Uptime: {}", uptime);
}

fn command_output(command: &str, args: &[&str]) -> String {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }

        _ => String::from("unknown"),
    }
}
