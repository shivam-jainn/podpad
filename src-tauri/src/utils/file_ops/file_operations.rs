use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Serialize)]
struct Node {
    title: String,
    key: String,
    #[serde(rename = "isLeaf")]
    is_leaf: bool,
    children: Option<Vec<Node>>,
}

impl Node {
    fn new(title: String, key: String, is_leaf: bool) -> Self {
        Node {
            title,
            key,
            is_leaf,
            children: if is_leaf { None } else { Some(Vec::new()) },
        }
    }

    fn add_child(&mut self, child: Node) {
        if let Some(children) = &mut self.children {
            children.push(child);
        }
    }
}

#[tauri::command]
pub fn display_folder_tree(workspace: &str) -> serde_json::Value {
    let root_path = PathBuf::from(workspace);
    let root_node = traverse_dir(&root_path, &root_path);
    json!(root_node)
}

fn traverse_dir(path: &Path, root_path: &Path) -> Node {
    let title = path
        .strip_prefix(root_path)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();
    let key = path.to_string_lossy().into_owned();
    let mut node = Node::new(title, key, false);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let child_node = traverse_dir(&entry_path, root_path);
                node.add_child(child_node);
            } else {
                let file_name = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let file_key = entry_path.to_string_lossy().into_owned();
                let file_node = Node::new(file_name, file_key, true);
                node.add_child(file_node);
            }
        }
    }

    node.is_leaf = node.children.as_ref().map_or(true, Vec::is_empty);
    node
}
