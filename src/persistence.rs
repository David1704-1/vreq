use crate::http::Request;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A named collection of HTTP requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<SavedRequest>,
}

/// A saved HTTP request with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub name: String,
    pub description: Option<String>,
    pub request: Request,
    pub created_at: String,  // ISO 8601 timestamp
    pub updated_at: String,
}

impl SavedRequest {
    pub fn new(name: String, request: Request) -> Self {
        // TODO: Use actual timestamp (add chrono crate)
        // let now = chrono::Utc::now().to_rfc3339();
        let now = String::from("2024-01-01T00:00:00Z");
        Self {
            name,
            description: None,
            request,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// Get the default directory for storing collections
pub fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // TODO: Implement platform-specific data directory lookup
    // On Linux: ~/.local/share/vreq/
    // On macOS: ~/Library/Application Support/vreq/
    // On Windows: %APPDATA%/vreq/
    //
    // You can use the `dirs` crate for this:
    // let data_dir = dirs::data_dir()
    //     .ok_or("Could not find data directory")?
    //     .join("vreq");
    //
    // std::fs::create_dir_all(&data_dir)?;
    // Ok(data_dir)

    Err("Data directory lookup not yet implemented".into())
}

/// Load a collection from disk
pub fn load_collection(name: &str) -> Result<Collection, Box<dyn std::error::Error>> {
    // TODO: Implement collection loading
    // Steps:
    // 1. Get data directory
    // 2. Construct file path: {data_dir}/collections/{name}.json
    // 3. Read file contents
    // 4. Deserialize JSON to Collection struct
    // 5. Return collection
    //
    // Example:
    // let data_dir = get_data_dir()?;
    // let file_path = data_dir.join("collections").join(format!("{}.json", name));
    // let contents = std::fs::read_to_string(file_path)?;
    // let collection: Collection = serde_json::from_str(&contents)?;
    // Ok(collection)

    Err("Collection loading not yet implemented".into())
}

/// Save a collection to disk
pub fn save_collection(collection: &Collection) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement collection saving
    // Steps:
    // 1. Get data directory
    // 2. Create collections subdirectory if it doesn't exist
    // 3. Construct file path: {data_dir}/collections/{name}.json
    // 4. Serialize Collection to JSON
    // 5. Write to file
    //
    // Example:
    // let data_dir = get_data_dir()?;
    // let collections_dir = data_dir.join("collections");
    // std::fs::create_dir_all(&collections_dir)?;
    //
    // let file_path = collections_dir.join(format!("{}.json", collection.name));
    // let json = serde_json::to_string_pretty(collection)?;
    // std::fs::write(file_path, json)?;
    // Ok(())

    Err("Collection saving not yet implemented".into())
}

/// List all available collections
pub fn list_collections() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: Implement collection listing
    // Steps:
    // 1. Get data directory
    // 2. Read collections subdirectory
    // 3. Filter for .json files
    // 4. Extract file names (without .json extension)
    // 5. Return list of collection names
    //
    // Example:
    // let data_dir = get_data_dir()?;
    // let collections_dir = data_dir.join("collections");
    //
    // if !collections_dir.exists() {
    //     return Ok(Vec::new());
    // }
    //
    // let mut names = Vec::new();
    // for entry in std::fs::read_dir(collections_dir)? {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.extension().and_then(|s| s.to_str()) == Some("json") {
    //         if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
    //             names.push(name.to_string());
    //         }
    //     }
    // }
    // Ok(names)

    Err("Collection listing not yet implemented".into())
}
