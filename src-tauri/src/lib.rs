use std::env;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! 你正在使用 Rust 函数!", name)
}

#[tauri::command]
fn get_os_username() -> String {
    let uname = env::var("USERNAME").unwrap();
    println!("用户名：{}", uname);
    return uname;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_os_username])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
