#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rclone;

use rclone::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_remotes,
            list_dir,
            copy_items,
            move_items,
            delete_items,
            get_jobs,
            get_job,
            config_list,
            config_create,
            config_reconnect,
            config_delete,
            check_rclone_version,
            install_rclone,
            update_rclone
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
