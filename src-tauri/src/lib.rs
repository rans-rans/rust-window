use sysinfo::System;

mod ram;
use ram::listen_to_ram;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_dialog::{MessageDialogButtons, MessageDialogKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // Register the commands that can be invoked from the frontend.
        .invoke_handler(tauri::generate_handler![get_system_info, open_ram_window])
        // Listen to RAM updates in the background and send them to the frontend when they occur.
        .setup(listen_to_ram)
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let app = window.app_handle().clone();
                    let _ = tauri_plugin_dialog::DialogExt::dialog(&app)
                        .message("Are you sure you want to close this window?")
                        .title("Confirm Close")
                        .kind(MessageDialogKind::Warning)
                        .buttons(MessageDialogButtons::OkCancel)
                        .show( move |result| {
                            if result {
                                app.exit(0);
                            }
                        });
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_system_info() -> OsInfo {
    OsInfo {
        system_name: System::name().unwrap_or(String::from("Unknown")),
        kernel_version: System::kernel_version().unwrap_or(String::from("Unknown")),
        host_name: System::host_name().unwrap_or(String::from("Unknown")),
        os_version: System::os_version().unwrap_or(String::from("Unknown")),
        cpu_arch: System::cpu_arch(),
    }
}

#[tauri::command]
fn open_ram_window(app: AppHandle) {
    let _ = WebviewWindowBuilder::new(&app, "ram-details", WebviewUrl::App("ram-info".into()))
        .title("RAM Details")
        .inner_size(600.0, 400.0)
        .build();
}

#[derive(Clone, serde::Serialize, Debug)]
struct OsInfo {
    system_name: String,
    kernel_version: String,
    host_name: String,
    os_version: String,
    cpu_arch: String,
}
