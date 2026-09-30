//! Detects which application currently has keyboard focus.
//!
//! Windows, macOS and Linux/X11 work out of the box via `x-win`.
//! GNOME on Wayland needs the small `x-win` GNOME Shell extension, which we
//! install on first start (it becomes active after the next login).

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ActiveApp {
    /// Executable name without extension, e.g. `code`, `firefox`.
    pub exec: String,
    /// Human-readable process/app name as reported by the OS.
    pub name: String,
    /// Window title.
    pub title: String,
}

pub fn active_app() -> Result<ActiveApp, String> {
    let win = x_win::get_active_window().map_err(|e| e.to_string())?;
    let exec = win.info.exec_name.trim_end_matches(".exe").to_string();
    Ok(ActiveApp {
        exec,
        name: win.info.name,
        title: win.title,
    })
}

pub fn is_gnome_wayland() -> bool {
    cfg!(target_os = "linux")
        && std::env::var("XDG_SESSION_TYPE").is_ok_and(|v| v == "wayland")
        && std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|v| v.to_lowercase().contains("gnome"))
}

/// Status of the GNOME helper extension, shown in the overlay if something is missing.
#[derive(Debug, Clone, Serialize)]
pub enum ExtensionStatus {
    NotNeeded,
    Ready,
    /// Freshly installed – user has to log out and back in once.
    NeedsRelogin,
    Error(String),
}

pub fn ensure_gnome_extension() -> ExtensionStatus {
    if !is_gnome_wayland() {
        return ExtensionStatus::NotNeeded;
    }
    if x_win::is_enabled_extension().unwrap_or(false) {
        return ExtensionStatus::Ready;
    }
    if !x_win::is_installed_extension().unwrap_or(false) {
        if let Err(e) = x_win::install_extension() {
            return ExtensionStatus::Error(e.to_string());
        }
    }
    // Enabling only succeeds once GNOME Shell has picked up the extension (after re-login).
    match x_win::enable_extension() {
        Ok(true) if x_win::is_enabled_extension().unwrap_or(false) => ExtensionStatus::Ready,
        _ => ExtensionStatus::NeedsRelogin,
    }
}
