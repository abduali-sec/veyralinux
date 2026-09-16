use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use tar::Archive;

#[derive(Debug)]
pub struct RepoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub filename: String,
    pub sha256: String,
}

pub fn find_package(query: &str) -> Result<Option<RepoPackage>, String> {
    let file = File::open("/tmp/veyra.db.tar.gz")
        .map_err(|e| format!("cannot open repository database: {}", e))?;

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for entry in archive
        .entries()
        .map_err(|e| format!("cannot read repository database: {}", e))?
    {
        let mut entry = entry.map_err(|e| e.to_string())?;

        let path = entry.path().map_err(|e| e.to_string())?;

        if !path.to_string_lossy().ends_with("/desc") {
            continue;
        }

        let mut text = String::new();
        entry
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;

        let fields = parse_desc(&text);

        let name = fields.get("NAME").cloned().unwrap_or_default();
        let version = fields.get("VERSION").cloned().unwrap_or_default();
        let description = fields.get("DESC").cloned().unwrap_or_default();
        let filename = fields
            .get("FILENAME")
            .cloned()
            .unwrap_or_else(|| format!("{}-{}.pkg.tar.zst", name, version));

        let sha256 = fields
            .get("SHA256SUM")
            .cloned()
            .unwrap_or_default();

        if name == query {
            return Ok(Some(RepoPackage {
                name,
                version,
                description,
                filename,
                sha256,
            }));
        }
    }

    Ok(None)
}

pub fn search(query: &str) -> Result<Vec<RepoPackage>, String> {
    let file = File::open("/tmp/veyra.db.tar.gz")
        .map_err(|e| format!("cannot open repository database: {}", e))?;

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for entry in archive
        .entries()
        .map_err(|e| format!("cannot read repository database: {}", e))?
    {
        let mut entry = entry.map_err(|e| e.to_string())?;

        let path = entry.path().map_err(|e| e.to_string())?;

        if !path.to_string_lossy().ends_with("/desc") {
            continue;
        }

        let mut text = String::new();
        entry
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;

        let fields = parse_desc(&text);

        let name = fields.get("NAME").cloned().unwrap_or_default();
        let version = fields.get("VERSION").cloned().unwrap_or_default();
        let description = fields.get("DESC").cloned().unwrap_or_default();
        let filename = fields
            .get("FILENAME")
            .cloned()
            .unwrap_or_else(|| format!("{}-{}.pkg.tar.zst", name, version));

        let sha256 = fields
            .get("SHA256SUM")
            .cloned()
            .unwrap_or_default();

        if name.to_lowercase().contains(&query)
            || description.to_lowercase().contains(&query)
        {
            results.push(RepoPackage {
                name,
                version,
                description,
                filename,
                sha256,
            });
        }
    }

    Ok(results)
}

fn parse_desc(text: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let mut current = None;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with('%') && line.ends_with('%') {
            current = Some(line.trim_matches('%').to_string());
            continue;
        }

        if let Some(key) = &current {
            map.insert(key.clone(), line.to_string());
            current = None;
        }
    }

    map
}
