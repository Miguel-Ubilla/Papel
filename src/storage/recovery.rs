use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::model::document::DocumentMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabSnapshot {
    pub metadata: DocumentMetadata,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub tabs: Vec<TabMetadataSnapshot>,
    pub active_index: usize,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabMetadataSnapshot {
    pub metadata: DocumentMetadata,
    pub filename: String,
}

pub struct RecoveryManager {
    recovery_dir: PathBuf,
}

impl RecoveryManager {
    pub fn new() -> Result<Self, String> {
        let state_dir = dirs::state_dir()
            .or_else(|| dirs::data_local_dir())
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        let recovery_dir = state_dir.join("papel").join("recovery");

        Self::with_dir(recovery_dir)
    }

    pub fn with_dir(recovery_dir: PathBuf) -> Result<Self, String> {
        if let Err(e) = fs::create_dir_all(&recovery_dir) {
            return Err(format!("Error al crear directorio de recuperación: {}", e));
        }

        Ok(Self { recovery_dir })
    }

    /// Guarda de forma atómica (primero .tmp, luego rename) todos los documentos activos.
    pub fn save_session_atomic(&self, tabs: &[TabSnapshot], active_index: usize) -> Result<(), String> {
        let mut tab_snapshots = Vec::new();

        // 1. Guardar cada archivo de contenido de forma atómica
        for tab in tabs {
            let filename = format!("{}.txt", tab.metadata.id);
            let target_path = self.recovery_dir.join(&filename);
            let temp_path = self.recovery_dir.join(format!("{}.txt.tmp", tab.metadata.id));

            // Escribir en temporal y renombrar atómicamente
            if let Err(e) = Self::atomic_write_file(&temp_path, &target_path, tab.content.as_bytes()) {
                eprintln!("Advertencia al autoguardar pestaña {}: {}", tab.metadata.id, e);
            }

            tab_snapshots.push(TabMetadataSnapshot {
                metadata: tab.metadata.clone(),
                filename,
            });
        }

        // 2. Guardar el archivo session.json atómicamente
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let session = SessionSnapshot {
            tabs: tab_snapshots,
            active_index,
            timestamp: now,
        };

        let session_json = serde_json::to_string_pretty(&session)
            .map_err(|e| format!("Error serializando sesión: {}", e))?;

        let session_target = self.recovery_dir.join("session.json");
        let session_temp = self.recovery_dir.join("session.json.tmp");

        Self::atomic_write_file(&session_temp, &session_target, session_json.as_bytes())?;

        Ok(())
    }

    /// Comprueba si existe una sesión anterior que deba restaurarse (e.g. tras corte o crash).
    pub fn load_recovered_session(&self) -> Option<(Vec<TabSnapshot>, usize)> {
        let session_path = self.recovery_dir.join("session.json");
        if !session_path.exists() {
            return None;
        }

        let mut file = match File::open(&session_path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            return None;
        }

        let session: SessionSnapshot = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(_) => return None,
        };

        if session.tabs.is_empty() {
            return None;
        }

        let mut recovered_tabs = Vec::new();
        for tab_meta in session.tabs {
            let tab_file_path = self.recovery_dir.join(&tab_meta.filename);
            let tab_content = if tab_file_path.exists() {
                fs::read_to_string(&tab_file_path).unwrap_or_default()
            } else if let Some(ref orig_path) = tab_meta.metadata.file_path {
                fs::read_to_string(orig_path).unwrap_or_default()
            } else {
                String::new()
            };

            recovered_tabs.push(TabSnapshot {
                metadata: tab_meta.metadata,
                content: tab_content,
            });
        }

        if recovered_tabs.is_empty() {
            None
        } else {
            Some((recovered_tabs, session.active_index))
        }
    }

    /// Elimina el archivo temporal de una pestaña cuando el usuario la cierra intencionadamente.
    pub fn remove_tab_recovery(&self, id: &str) {
        let file_path = self.recovery_dir.join(format!("{}.txt", id));
        let _ = fs::remove_file(file_path);
        let tmp_path = self.recovery_dir.join(format!("{}.txt.tmp", id));
        let _ = fs::remove_file(tmp_path);
    }

    /// Limpia la sesión cuando el usuario cierra la aplicación normalmente sin pestañas sin guardar.
    #[allow(dead_code)]
    pub fn clear_session(&self) {
        let session_path = self.recovery_dir.join("session.json");
        let _ = fs::remove_file(session_path);
    }

    /// Operación de escritura atómica: escribe en .tmp y hace rename.
    fn atomic_write_file(temp_path: &Path, target_path: &Path, bytes: &[u8]) -> Result<(), String> {
        let mut file = File::create(temp_path)
            .map_err(|e| format!("No se pudo crear archivo temporal {:?}: {}", temp_path, e))?;
        file.write_all(bytes)
            .map_err(|e| format!("Error escribiendo datos: {}", e))?;
        file.sync_all()
            .map_err(|e| format!("Error sincronizando a disco (fsync): {}", e))?;
        drop(file);

        fs::rename(temp_path, target_path)
            .map_err(|e| format!("Error al renombrar atómicamente a {:?}: {}", target_path, e))?;

        Ok(())
    }
}
