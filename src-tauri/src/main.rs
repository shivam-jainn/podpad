mod utils;

use utils::file_ops::file_operations::display_file;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![display_file])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}

