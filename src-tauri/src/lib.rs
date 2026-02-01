use tauri_plugin_log::{Target, TargetKind};

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Debug)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::detect_system,
            commands::get_service_status,
            commands::start_service,
            commands::stop_service,
            commands::get_php_versions,
            commands::switch_php_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
