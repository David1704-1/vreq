use crate::http::Request;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::{create_dir_all, read, write, read_dir};
use chrono::offset::Utc;
use std::time::SystemTime;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub saved_request: SavedRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub request: Request,
    pub created_at: String,
    pub updated_at: String,
}

impl SavedRequest {
    pub fn new(request: Request) -> Self {
        let now = SystemTime::now();
        let datetime: chrono::DateTime<Utc> = now.into();
        Self {
            request,
            created_at: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

impl Collection {
    pub fn new(name: String, request: Request) -> Self {
        Collection {
            name,
            saved_request: SavedRequest::new(request)
        }
    }
}

pub fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME").unwrap_or("".to_string());
    let data_dir = PathBuf::from(format!("{}/.local/vreq/collections", home_dir));
    create_dir_all(data_dir.clone())?;
    Ok(data_dir)
}

pub fn load_collection(name: &str) -> Result<Collection, Box<dyn std::error::Error>> {
    let file_name = format!("{}.json", name);
    let mut data_dir = get_data_dir()?;
    data_dir.push(file_name);
    let contents = read(data_dir)?;
    let collection: Collection = serde_json::from_slice(&contents)?;
    Ok(collection)
}

pub fn save_collection(collection: &Collection) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}.json", collection.name);
    let mut data_dir = get_data_dir()?;
    data_dir.push(file_name);
    let contents = serde_json::to_string(collection)?;
    write(&data_dir, contents)?;
    Ok(())
}

pub fn list_collections() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file_names: Vec<String> = Vec::new();
    let data_dir = get_data_dir()?;
    let read_dir = read_dir(data_dir)?;
    for entry in read_dir {
        let file_name = entry?.file_name();
        if let Some(name) = file_name.to_str() {
            file_names.push(name.to_owned());
        }
    };

    Ok(file_names)
}
