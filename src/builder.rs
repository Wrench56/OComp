use axum::extract::Multipart;
use std::fs;
use uuid::Uuid;

use crate::config::get_config_dir;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref UPLOAD_DIR: Arc<Mutex<Option<PathBuf>>> = Arc::new(Mutex::new(None));
}

fn set_upload_dir(path: PathBuf) {
    let mut upload_dir = UPLOAD_DIR.lock().unwrap();
    *upload_dir = Some(path);
}

pub fn get_upload_dir() -> PathBuf {
    let upload_dir = UPLOAD_DIR.lock().unwrap();
    upload_dir.clone().expect("Upload directory is not set")
}

pub async fn init() {
    let upload_dir = get_config_dir().join("uploads");

    if upload_dir.exists() {
        fs::remove_dir_all(&upload_dir).expect("Failed to clear existing upload directory");
    }

    fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");
    set_upload_dir(upload_dir);
}

pub async fn build(_build_command: &str, mut multipart: Multipart) -> Result<String, String> {
    let build_dir_path = create_build_directory();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("Error reading field: {}", e))?
    {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "uploaded_file".to_string());
        let file_path = get_upload_dir()
            .join(build_dir_path.clone())
            .join(file_name);

        let data = field
            .bytes()
            .await
            .map_err(|e| format!("Error reading file: {}", e))?;
        fs::write(&file_path, data).map_err(|e| format!("Failed to save file: {}", e))?;
    }

    Err("No file uploaded".to_string())
}

fn create_build_directory() -> PathBuf {
    let upload_dir = get_upload_dir();
    let uuid = Uuid::new_v4().to_string();
    let build_dir = upload_dir.join(format!("build-{}", uuid));

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    build_dir
}
