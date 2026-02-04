use crate::http::Request;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<SavedRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub name: String,
    pub description: Option<String>,
    pub request: Request,
    pub created_at: String,
    pub updated_at: String,
}

impl SavedRequest {
    pub fn new(name: String, request: Request) -> Self {
        // TODO: Use actual timestamp (add chrono crate)
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

pub fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // TODO: Implement platform-specific data directory lookup
    Err("Data directory lookup not yet implemented".into())
}

pub fn load_collection(name: &str) -> Result<Collection, Box<dyn std::error::Error>> {
    // TODO: Implement collection loading
    Err("Collection loading not yet implemented".into())
}

pub fn save_collection(collection: &Collection) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement collection saving
    Err("Collection saving not yet implemented".into())
}

pub fn list_collections() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: Implement collection listing
    Err("Collection listing not yet implemented".into())
}
