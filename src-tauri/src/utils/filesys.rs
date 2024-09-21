use std::fs::File;
use std::io;
use serde_json::json;

#[tauri::command]
pub fn add_folder(workspace: &str, path: &str) -> String {
    let folder_path = format!("{}/{}", workspace, path);
    if std::fs::metadata(&folder_path).is_ok() {
        return json!({"success": false, "error": "Folder already exists"}).to_string();
    }

    let result = std::fs::create_dir(&folder_path);

    match result {
        Ok(_) => json!({"success": true}).to_string(),
        Err(e) => json!({"success": false, "error": e.to_string()}).to_string(),
    }
}

#[tauri::command]
pub fn add_file(workspace: &str, path: &str) -> String {
    let file_path = format!("{}/{}", workspace, path);
    if std::fs::metadata(&file_path).is_ok() {
        return json!({"success": false, "error": "File already exists"}).to_string();
    }

    let result = File::create(&file_path);
    
    match result {
        Ok(_) => json!({"success": true}).to_string(),
        Err(e) => json!({"success": false, "error": e.to_string()}).to_string(),
    }
}

#[tauri::command]
pub fn delete_node(workspace: &str, path: &str) -> String {
    let node_path = format!("{}/{}", workspace, path);
    
    let result = if std::fs::metadata(&node_path).map(|m| m.is_dir()).unwrap_or(false) {
        std::fs::remove_dir_all(&node_path)
    } else {
        std::fs::remove_file(&node_path)
    };

    match result {
        Ok(_) => json!({"success": true}).to_string(),
        Err(e) => json!({"success": false, "error": e.to_string()}).to_string(),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sequential_operations() {
        let workspace = ".";

        // Step 1: Create the main folder
        let folder_path = "test_workspace";
        let result = add_folder(workspace, folder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to create main folder");

        // Step 2: Create a subfolder inside the main folder
        let subfolder_path = "test_workspace/test_subfolder";
        let result = add_folder(workspace, subfolder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to create subfolder");

        // Step 3: Try creating the same subfolder (should fail)
        let result = add_folder(workspace, subfolder_path);
        assert_eq!(result, r#"{"success":false,"error":"Folder already exists"}"#, "Subfolder already exists check failed");

        // Step 4: Create a file in the main folder
        let file_path = "test_workspace/test_file.txt";
        let result = add_file(workspace, file_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to create file in main folder");

        // Step 5: Try creating the same file (should fail)
        let result = add_file(workspace, file_path);
        assert_eq!(result, r#"{"success":false,"error":"File already exists"}"#, "File in main folder already exists check failed");

        // Step 6: Create a file in the subfolder
        let subfile_path = "test_workspace/test_subfolder/test_subfile.txt";
        let result = add_file(workspace, subfile_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to create file in subfolder");

        // Step 7: Try creating the same file in subfolder (should fail)
        let result = add_file(workspace, subfile_path);
        assert_eq!(result, r#"{"success":false,"error":"File already exists"}"#, "File in subfolder already exists check failed");

        // Step 8: Delete the file in the subfolder
        let result = delete_node(workspace, subfile_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete file in subfolder");

        // Step 9: Delete the subfolder
        let result = delete_node(workspace, subfolder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete subfolder");

        // Step 10: Delete the file in the main folder
        let result = delete_node(workspace, file_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete file in main folder");

        // Step 11: Delete the main folder
        let result = delete_node(workspace, folder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete main folder");
    }
}
