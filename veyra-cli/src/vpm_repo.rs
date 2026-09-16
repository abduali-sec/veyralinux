use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

#[derive(Debug)]
pub struct RepoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub fn search(query: &str) -> Result<Vec<RepoPackage>, String> {
    let file = File::open("/tmp/veyra.db.tar.gz")
        .map_err(|e| format!("cannot open repository database: {}", e))?;

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let mut results = Vec::new();

    let entries = archive
        .entries()
        .map_err(|e| format!("cannot read repository database: {}", e))?;

    for entry in entries {
        let mut entry = entry.map_err(|e| e.to_string())?;

        let path = entry.path().map_err(|e| e.to_string())?;

        if !path.to_string_lossy().ends_with("/desc") {
            continue;
        }

        let mut text = String::new();
        std::io::Read::read_to_string(&mut entry, &mut text)
            .map_err(|e| e.to_string())?;

        let fields = parse_desc(&text);

        let name = fields.get("NAME").cloned().unwrap_or_default();
        let version = fields.get("VERSION").cloned().unwrap_or_default();
        let description = fields.get("DESC").cloned().unwrap_or_default();

        let q = query.to_lowercase();

        if name.to_lowercase().contains(&q)
            || description.to_lowercase().contains(&q)
        {
            results.push(RepoPackage {
                name,
                version,
                description,
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
