use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa el estado y metadatos de un documento abierto en una pestaña.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: String,
    pub title: String,
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,
}

impl DocumentMetadata {
    pub fn new_empty(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            file_path: None,
            is_dirty: false,
        }
    }

    pub fn from_file(file_path: PathBuf, title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            file_path: Some(file_path),
            is_dirty: false,
        }
    }
}

