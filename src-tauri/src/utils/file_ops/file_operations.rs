use serde_json::{json, Value};
use std::fs;

#[tauri::command]
pub fn display_file(path: &str) -> Value {
    match fs::read_to_string(path) {
        Ok(file_contents) => {
            json!({
                "success": true,
                "file_contents": file_contents
            })
        }
        Err(e) => {
            json!({
                "success": false,
                "error": format!("Error reading file: {}", e)
            })
        }
    }
}
