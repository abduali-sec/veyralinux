use std::process::Command;

const API_URL: &str = "https://veyra-api.abdualialderson.workers.dev";
const REPO_URL: &str =
    "https://raw.githubusercontent.com/abduali-sec/veyra-repo/main";

pub fn repository() -> bool {
    let db_url = format!("{}/veyra.db.tar.gz", REPO_URL);

    println!("Veyra Repository");
    println!("Database: {}", db_url);

    match Command::new("curl")
        .args(["-fsSL", &db_url, "-o", "/tmp/veyra.db.tar.gz"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✓ Repository database downloaded.");
            true
        }
        _ => {
            eprintln!("✗ Failed to download repository database.");
            false
        }
    }
}

pub fn info(package: &str) -> bool {
    let url = format!("{}/packages/{}", API_URL, package);

    match Command::new("curl")
        .args(["-fsSL", &url])
        .output()
    {
        Ok(output) if output.status.success() => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            true
        }
        _ => {
            eprintln!("vpm: package '{}' not found", package);
            false
        }
    }
}
