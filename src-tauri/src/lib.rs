use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn write_test(content: &str) -> Result<String, String> {
    let mut file = std::fs::File::create("test.txt")
        .map_err(|e| e.to_string())?;

    file.write(content.as_bytes()).map_err(|e| e.to_string())?;

    Ok("test created successfully".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, write_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
