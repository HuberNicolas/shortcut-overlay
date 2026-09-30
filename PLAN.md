# Plan: Shortcut Overlay

## Entscheidungen

- **Tauri 2** (Rust + Vue/TypeScript) statt Python/Qt oder Electron: kleine Binaries,
  Tray/Hotkey/transparente Fenster eingebaut, UI in bekanntem Vue.
- **Fokus-Erkennung** über die Rust-Crate `x-win` (Windows, macOS, X11, GNOME-Wayland via Extension).
- **Toggle** über Single-Instance: `shortcut-overlay --toggle` schickt das Signal an die laufende
  Instanz → funktioniert auch unter Wayland über einen GNOME-Shortcut.
- **Unter GNOME-Wayland** läuft das Fenster über XWayland (`GDK_BACKEND=x11`), damit
  Always-on-top und Zentrieren funktionieren.
- **Profile** als YAML, `Mod` = Ctrl/⌘, Overrides pro OS, `os:`-Filter, `learn: true`.

## Stand

- [x] Grundgerüst, Overlay-Design (HUD, Filter, Tab App ⇄ System)
- [x] 8 eingebaute Profile (System, VS Code, Firefox, Chromium, Terminal, Dateimanager, JetBrains, Slack)
- [x] Eigene Profile im Config-Ordner, Tray-Menü
- [x] Theming (Haupt-/Zweitfarbe, Presets, in `config.yaml` gespeichert)
- [x] Gedrückte Tasten leuchten auf, exakte Kombination hebt die Zeile hervor
- [x] CI + Release-Workflow für alle drei Systeme
- [ ] Auf Ubuntu/Wayland end-to-end testen
- [ ] Auf macOS und Windows testen

## Später

- Lernmodus ausbauen: nur `learn`-Shortcuts anzeigen, Shortcut im Overlay per Klick markieren
- Profile nach Fenstertitel (z. B. Web-Apps im Browser: GitHub, Gmail)
- Autostart beim Login
- Eigenes App-Icon
