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

#[tauri::command]
pub fn access_file(workspace: &str, path: &str) -> Result<Vec<u8>, String> {
    let file_path = format!("{}/{}", workspace, path);
    let result = File::open(&file_path);

    match result {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(e) = std::io::Read::read_to_end(&mut file, &mut buffer) {
                return Err(json!({"success": false, "error": e.to_string()}).to_string());
            }
            Ok(buffer)
        }
        Err(e) => Err(json!({"success": false, "error": e.to_string()}).to_string()),
    }
}

#[tauri::command]
pub fn rename_node(workspace: &str, old_path: &str, new_path: &str) -> String {
    let old_node_path = format!("{}/{}", workspace, old_path);
    let new_node_path = format!("{}/{}", workspace, new_path);

    let result = if std::fs::metadata(&old_node_path).map(|m| m.is_dir()).unwrap_or(false) {
        std::fs::rename(&old_node_path, &new_node_path)
    } else {
        std::fs::rename(&old_node_path, &new_node_path)
    };

    match result {
        Ok(_) => json!({"success": true}).to_string(),
        Err(e) => json!({"success": false, "error": e.to_string()}).to_string(),
    }
}

#[tauri::command]
pub fn list_nodes(workspace: &str) -> String {
    let folder_path = workspace; // Corrected folder_path assignment
    let result = std::fs::read_dir(&folder_path);

    match result {
        Ok(entries) => {
            let mut nodes = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    let node_type = if entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
                        "folder"
                    } else {
                        "file"
                    };
                    let node_name = entry.file_name().into_string().unwrap_or_default();
                    nodes.push(json!({"type": node_type, "name": node_name}));
                }
            }
            json!({"success": true, "nodes": nodes}).to_string()
        }
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

        // Step 8: Rename the file in the subfolder
        let new_subfile_path = "test_workspace/test_subfolder/test_subfile_renamed.txt";
        let result = rename_node(workspace, subfile_path, new_subfile_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to rename file in subfolder");

        // Step 9: Rename the subfolder
        let new_subfolder_path = "test_workspace/test_subfolder_renamed";
        let result = rename_node(workspace, subfolder_path, new_subfolder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to rename subfolder");
        
        // Step 10: Delete the file in the subfolder
        let subfile_path = "test_workspace/test_subfolder_renamed/test_subfile_renamed.txt";
        let result = delete_node(workspace, subfile_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete file in subfolder");

        // Step 11: Delete the subfolder
        let subfolder_path = "test_workspace/test_subfolder_renamed";
        let result = delete_node(workspace, subfolder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete subfolder");

        // Step 12: Delete the file in the main folder
        let result = delete_node(workspace, file_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete file in main folder");

        // Step 13: Delete the main folder
        let result = delete_node(workspace, folder_path);
        assert_eq!(result, r#"{"success":true}"#, "Failed to delete main folder");
    }

    #[test]
    fn test_file_access() {
        let workspace = ".";
        let file_path = "test_data/Hello.txt";
    
        let result = access_file(workspace, file_path);
        assert!(result.is_ok(), "Failed to access file");
    
        // Convert the Vec<u8> to a string before comparison
        let content = String::from_utf8(result.unwrap()).expect("Failed to convert to string");
        assert_eq!(content, "hello world", "File content mismatch");
    }
    
    #[test]
    fn test_list_nodes() {
        let workspace = "test_data/file_list_folder";
        let result = list_nodes(workspace);
        assert!(result.contains(r#""type":"file","name":"NewFiles.py""#), "NewFiles.py not found");
        assert!(result.contains(r#""type":"file","name":"NewFiles.txt""#), "NewFiles.txt not found");
    }
    
}
