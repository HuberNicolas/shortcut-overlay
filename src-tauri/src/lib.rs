mod focus;
mod shortcuts;

use focus::{ActiveApp, ExtensionStatus};
use serde::{Deserialize, Serialize};
use shortcuts::{Library, Profile};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_opener::OpenerExt;

/// Passed by a second launch (e.g. a GNOME custom shortcut) to toggle the running instance.
const TOGGLE_ARG: &str = "--toggle";
const DEFAULT_HOTKEY: &str = "CommandOrControl+Alt+Slash";
const OVERLAY: &str = "overlay";

/// Everything the overlay needs to render, sent on every show.
#[derive(Clone, Serialize)]
struct OverlayState {
    app: Option<ActiveApp>,
    /// Profile of the focused app, if one matches.
    profile: Option<Profile>,
    /// Generic OS shortcuts, always available (Tab switches to it).
    system: Option<Profile>,
    error: Option<String>,
    warnings: Vec<String>,
    extension: ExtensionStatus,
    os: &'static str,
    hotkey: String,
    shortcuts_dir: String,
}

#[derive(Default, Deserialize)]
struct Config {
    hotkey: Option<String>,
}

struct AppState {
    current: Mutex<Option<OverlayState>>,
    extension: Mutex<ExtensionStatus>,
    hotkey: String,
}

fn config_dir(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn shortcuts_dir(app: &AppHandle) -> PathBuf {
    config_dir(app).join("shortcuts")
}

fn build_state(app: &AppHandle) -> OverlayState {
    let st = app.state::<AppState>();
    let dir = shortcuts_dir(app);
    // Re-read on every show so edits to user files apply without a restart.
    let lib = Library::load(Some(&dir));
    let (active, error) = match focus::active_app() {
        Ok(a) => (Some(a), None),
        Err(e) => (None, Some(e)),
    };
    let profile = active.as_ref().and_then(|a| lib.find(&a.exec, &a.name)).cloned();
    let extension = st.extension.lock().unwrap().clone();

    OverlayState {
        app: active,
        profile,
        system: lib.fallback().cloned(),
        error,
        warnings: lib.warnings,
        extension,
        os: std::env::consts::OS,
        hotkey: st.hotkey.clone(),
        shortcuts_dir: dir.to_string_lossy().into_owned(),
    }
}

fn toggle(app: &AppHandle) {
    let Some(win) = app.get_webview_window(OVERLAY) else {
        return;
    };
    if win.is_visible().unwrap_or(false) {
        let _ = win.hide();
        return;
    }
    // Query focus *before* showing, otherwise we'd detect ourselves.
    let state = build_state(app);
    *app.state::<AppState>().current.lock().unwrap() = Some(state.clone());
    let _ = win.emit("overlay://show", &state);
    let _ = win.center();
    let _ = win.show();
    let _ = win.set_focus();
}

#[tauri::command]
fn current_state(state: tauri::State<AppState>) -> Option<OverlayState> {
    state.current.lock().unwrap().clone()
}

#[tauri::command]
fn hide_overlay(app: AppHandle) {
    if let Some(win) = app.get_webview_window(OVERLAY) {
        let _ = win.hide();
    }
}

#[tauri::command]
fn open_shortcuts_dir(app: AppHandle) {
    let _ = app.opener().open_path(shortcuts_dir(&app).to_string_lossy(), None::<&str>);
}

/// Creates the user shortcut folder with a short how-to on first start.
fn init_user_dir(app: &AppHandle) {
    let dir = shortcuts_dir(app);
    let _ = std::fs::create_dir_all(&dir);
    let readme = dir.join("README.md");
    if !readme.exists() {
        let _ = std::fs::write(readme, include_str!("../shortcuts/USER_README.md"));
    }
}

fn load_config(app: &AppHandle) -> Config {
    std::fs::read_to_string(config_dir(app).join("config.yaml"))
        .ok()
        .and_then(|s| serde_yaml::from_str(&s).ok())
        .unwrap_or_default()
}

fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show overlay", true, None::<&str>)?;
    let folder = MenuItem::with_id(app, "folder", "Open shortcuts folder", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &folder, &quit])?;

    let mut tray = TrayIconBuilder::new()
        .tooltip("Shortcut Overlay")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => toggle(app),
            "folder" => open_shortcuts_dir(app.clone()),
            "quit" => app.exit(0),
            _ => {}
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // On GNOME/Wayland native windows can't stay on top or be centered;
    // running through XWayland fixes both. Overrides an inherited GDK_BACKEND=wayland.
    if focus::is_gnome_wayland() {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    tauri::Builder::default()
        // Must be the first plugin: a second launch just forwards its args here.
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if args.iter().any(|a| a == TOGGLE_ARG) {
                toggle(app);
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle().clone();
            init_user_dir(&handle);
            let hotkey = load_config(&handle).hotkey.unwrap_or_else(|| DEFAULT_HOTKEY.into());

            app.manage(AppState {
                current: Mutex::new(None),
                extension: Mutex::new(ExtensionStatus::NotNeeded),
                hotkey: hotkey.clone(),
            });

            // Global hotkeys don't work on Wayland – use a desktop shortcut running `--toggle` there.
            if let Err(e) = app.global_shortcut().register(hotkey.as_str()) {
                eprintln!("could not register hotkey {hotkey}: {e}");
            }

            setup_tray(&handle)?;

            // Installing/enabling the GNOME extension shells out, keep it off the main thread.
            std::thread::spawn(move || {
                let status = focus::ensure_gnome_extension();
                *handle.state::<AppState>().extension.lock().unwrap() = status;
                if std::env::args().any(|a| a == TOGGLE_ARG) {
                    toggle(&handle);
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == OVERLAY {
                match event {
                    WindowEvent::Focused(false) => {
                        let _ = window.hide();
                    }
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = window.hide();
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(tauri::generate_handler![current_state, hide_overlay, open_shortcuts_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
