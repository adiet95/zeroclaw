//! Tray menu event handling.

use tauri::{AppHandle, Manager, Runtime, menu::MenuEvent};

const GATEWAY_URL: &str = "http://127.0.0.1:42617/";

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "show" => show_main_window(app, None),
        "browser" => open_browser(),
        "chat" => show_main_window(app, Some("/agent")),
        "service-toggle" => {
            let state = app.state::<crate::state::SharedState>().inner().clone();
            let app = app.clone();
            tauri::async_runtime::spawn(crate::toggle_service(app, state));
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

fn open_browser() {
    let mut command = std::process::Command::new(if cfg!(windows) {
        "explorer.exe"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    });
    command.arg(GATEWAY_URL);
    command.stdin(std::process::Stdio::null());
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    let _ = command.spawn();
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>, navigate_to: Option<&str>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        if let Some(path) = navigate_to {
            let script = format!("window.location.hash = '{path}'");
            let _ = window.eval(&script);
        }
    }
}
