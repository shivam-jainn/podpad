mod utils;

use utils::file_ops::file_operations::{display_file, display_folder_tree};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![display_file, display_folder_tree])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}

