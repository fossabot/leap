use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::dirs;

const DB_URL: &str = "https://appimage.github.io/search.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub url: String,
}

// Fetches the catalog from the appimage database
// If the catalog is already present in the cache, it will be used instead
pub fn fetch_catalog() -> Result<Vec<Database>, Box<dyn std::error::Error>> {
    log::info!("Fetching catalog from {}", DB_URL);
    let mut path = dirs::cache_dir().unwrap();
    path.push("database.json");

    if path.exists() {
        log::info!("Using cached catalog");
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let db: Vec<Database> = serde_json::from_reader(reader)?;
        Ok(db)
    } else {
        log::info!("Downloading catalog");
        let resp = reqwest::blocking::get(DB_URL)?.text()?;
        let db: Vec<Database> = serde_json::from_str(&resp)?;
        log::info!("Saving catalog to cache");

        let mut path = dirs::cache_dir().unwrap();
        path.push("database.json");
        let mut file = std::fs::File::create(path)?;
        file.write_all(resp.as_bytes())?;

        Ok(db)
    }
}
