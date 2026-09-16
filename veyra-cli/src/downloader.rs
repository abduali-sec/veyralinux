use std::process::Command;

pub fn download(url: &str, output: &str) -> bool {
    println!("Veyra Downloader");
    println!("Downloading:");
    println!("  {}", url);
    println!("To:");
    println!("  {}", output);

    match Command::new("curl")
        .args(["-fL", url, "-o", output])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✓ Download completed.");
            true
        }

        Ok(_) => {
            eprintln!("✗ Download failed.");
            false
        }

        Err(error) => {
            eprintln!("✗ Failed to start downloader: {}", error);
            false
        }
    }
}
