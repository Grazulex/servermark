use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
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
        .setup(|app| {
            // Create tray menu items
            let open_i = MenuItem::with_id(app, "open", "Open ServerMark", true, None::<&str>)?;
            let separator1 = MenuItem::with_id(app, "sep1", "─────────────", false, None::<&str>)?;
            let start_all_i = MenuItem::with_id(app, "start_all", "Start All Containers", true, None::<&str>)?;
            let stop_all_i = MenuItem::with_id(app, "stop_all", "Stop All Containers", true, None::<&str>)?;
            let separator2 = MenuItem::with_id(app, "sep2", "─────────────", false, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            // Build menu
            let menu = Menu::with_items(
                app,
                &[
                    &open_i,
                    &separator1,
                    &start_all_i,
                    &stop_all_i,
                    &separator2,
                    &quit_i,
                ],
            )?;

            // Build tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("ServerMark")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "start_all" => {
                        log::info!("Start all containers requested from tray");
                        // TODO: Implement start all containers
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-start-all", ());
                        }
                    }
                    "stop_all" => {
                        log::info!("Stop all containers requested from tray");
                        // TODO: Implement stop all containers
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-stop-all", ());
                        }
                    }
                    "quit" => {
                        log::info!("Quit requested from tray");
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::detect_system,
            commands::get_service_status,
            commands::start_service,
            commands::stop_service,
            commands::get_php_versions,
            commands::switch_php_version,
            commands::install_php_version,
            commands::check_php_ppa,
            commands::add_php_ppa,
            commands::get_php_extensions,
            commands::install_php_with_extensions,
            commands::uninstall_php_version,
            // Docker/Podman commands
            commands::detect_container_runtime,
            commands::list_containers,
            commands::create_container,
            commands::start_container,
            commands::stop_container,
            commands::remove_container,
            commands::get_container_logs,
            // Laravel commands
            commands::detect_laravel_version,
            commands::get_latest_laravel_version,
            commands::upgrade_laravel,
            commands::create_laravel_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
