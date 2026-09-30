//! Shortcut profiles: one YAML file per application.
//!
//! Built-in profiles are compiled into the binary. Files in the user's
//! `<config dir>/shortcuts/` folder override a built-in profile with the same
//! `id`, or add new ones.

use serde::{Deserialize, Serialize};
use std::path::Path;

const BUILTIN: &[(&str, &str)] = &[
    ("system.yaml", include_str!("../shortcuts/system.yaml")),
    ("vscode.yaml", include_str!("../shortcuts/vscode.yaml")),
    ("firefox.yaml", include_str!("../shortcuts/firefox.yaml")),
    ("chromium.yaml", include_str!("../shortcuts/chromium.yaml")),
    ("terminal.yaml", include_str!("../shortcuts/terminal.yaml")),
    ("files.yaml", include_str!("../shortcuts/files.yaml")),
    ("jetbrains.yaml", include_str!("../shortcuts/jetbrains.yaml")),
    ("slack.yaml", include_str!("../shortcuts/slack.yaml")),
];

/// Profile shown when no other profile matches the focused app.
pub const FALLBACK_ID: &str = "system";

// ---- file format -----------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ProfileFile {
    id: String,
    name: String,
    #[serde(default, rename = "match")]
    matches: Vec<String>,
    groups: Vec<GroupFile>,
}

#[derive(Debug, Deserialize)]
struct GroupFile {
    name: String,
    shortcuts: Vec<ShortcutFile>,
}

#[derive(Debug, Deserialize)]
struct ShortcutFile {
    /// Platform-neutral keys, `Mod` = Ctrl (Linux/Windows) or Cmd (macOS).
    keys: String,
    mac: Option<String>,
    linux: Option<String>,
    windows: Option<String>,
    action: String,
    /// Marked as "I want to learn this" – highlighted in the overlay.
    #[serde(default)]
    learn: bool,
    /// Only show on these systems (`linux`, `macos`, `windows`).
    os: Option<Vec<String>>,
}

// ---- resolved data sent to the frontend ------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    #[serde(skip)]
    matches: Vec<String>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub name: String,
    pub shortcuts: Vec<Shortcut>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Shortcut {
    /// Key sequence: each step is a chord of keys, e.g. `[["Ctrl","K"],["Ctrl","S"]]`.
    pub keys: Vec<Vec<String>>,
    pub action: String,
    pub learn: bool,
}

pub struct Library {
    pub profiles: Vec<Profile>,
    /// Problems while reading user files, shown in the overlay.
    pub warnings: Vec<String>,
}

impl Library {
    pub fn load(user_dir: Option<&Path>) -> Self {
        let mut profiles = Vec::new();
        let mut warnings = Vec::new();

        for (file, src) in BUILTIN {
            match parse(src) {
                Ok(p) => profiles.push(p),
                Err(e) => warnings.push(format!("built-in {file}: {e}")),
            }
        }

        if let Some(dir) = user_dir {
            for (file, src) in read_yaml_files(dir) {
                match parse(&src) {
                    Ok(p) => match profiles.iter_mut().find(|q| q.id == p.id) {
                        Some(existing) => *existing = p,
                        None => profiles.push(p),
                    },
                    Err(e) => warnings.push(format!("{file}: {e}")),
                }
            }
        }

        Self { profiles, warnings }
    }

    /// Finds the profile for an app by comparing executable and app name
    /// (case-insensitive) against each profile's `match` list.
    pub fn find(&self, exec: &str, name: &str) -> Option<&Profile> {
        let (exec, name) = (exec.to_lowercase(), name.to_lowercase());
        self.profiles
            .iter()
            .filter(|p| p.id != FALLBACK_ID)
            .find(|p| p.matches.iter().any(|m| *m == exec || *m == name))
    }

    pub fn fallback(&self) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.id == FALLBACK_ID)
    }
}

fn read_yaml_files(dir: &Path) -> Vec<(String, String)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<_> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yaml" || x == "yml"))
        .collect();
    files.sort();
    files
        .into_iter()
        .filter_map(|p| {
            let src = std::fs::read_to_string(&p).ok()?;
            Some((p.file_name()?.to_string_lossy().into_owned(), src))
        })
        .collect()
}

fn parse(src: &str) -> Result<Profile, serde_yaml::Error> {
    let file: ProfileFile = serde_yaml::from_str(src)?;
    Ok(Profile {
        id: file.id,
        name: file.name,
        matches: file.matches.iter().map(|m| m.to_lowercase()).collect(),
        groups: file
            .groups
            .into_iter()
            .map(|g| Group {
                name: g.name,
                shortcuts: g
                    .shortcuts
                    .into_iter()
                    .filter(|s| {
                        s.os.as_ref()
                            .map_or(true, |os| os.iter().any(|o| o == std::env::consts::OS))
                    })
                    .map(resolve)
                    .collect(),
            })
            .collect(),
    })
}

fn resolve(s: ShortcutFile) -> Shortcut {
    let keys = match std::env::consts::OS {
        "macos" => s.mac,
        "windows" => s.windows,
        _ => s.linux,
    }
    .unwrap_or(s.keys);

    let modifier = if cfg!(target_os = "macos") { "Cmd" } else { "Ctrl" };
    Shortcut {
        keys: keys
            .split_whitespace()
            .map(|chord| {
                chord
                    .split('+')
                    .map(|k| if k == "Mod" { modifier.to_string() } else { k.to_string() })
                    .collect()
            })
            .collect(),
        action: s.action,
        learn: s.learn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_profiles_parse() {
        let lib = Library::load(None);
        assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);
        assert_eq!(lib.profiles.len(), BUILTIN.len());
        assert!(lib.fallback().is_some());
    }

    #[test]
    fn matches_case_insensitive() {
        let lib = Library::load(None);
        assert_eq!(lib.find("Code", "").map(|p| p.id.as_str()), Some("vscode"));
        assert!(lib.find("does-not-exist", "nope").is_none());
    }

    #[test]
    fn resolves_sequences_and_mod() {
        let s = resolve(ShortcutFile {
            keys: "Mod+K Mod+S".into(),
            mac: None,
            linux: None,
            windows: None,
            action: "x".into(),
            learn: false,
            os: None,
        });
        let m = if cfg!(target_os = "macos") { "Cmd" } else { "Ctrl" };
        assert_eq!(s.keys, vec![vec![m, "K"], vec![m, "S"]]);
    }
}
