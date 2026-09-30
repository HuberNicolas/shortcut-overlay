# Shortcut Overlay

Ein kleines Desktop-Tool für **Ubuntu, macOS und Windows**: Per Hotkey erscheint
ein HUD-Overlay mit den nützlichsten Tastenkürzeln des Programms, das gerade im
Fokus ist.

- Erkennt das fokussierte Programm (VS Code, Firefox, Chrome, Terminal, Dateimanager, JetBrains, Slack, …)
- Fällt auf System-Shortcuts zurück, wenn es kein Profil gibt – <kbd>Tab</kbd> wechselt jederzeit
- Tippen filtert live, <kbd>Esc</kbd> schliesst
- Gedrückte Tasten leuchten auf – hält man genau eine Kombination, wird die Zeile hervorgehoben
- Theming: Haupt- und Zweitfarbe wählbar (Presets oder Farbregler unter „◐ theme“)
- Eigene Profile als YAML, Shortcuts zum Lernen mit `learn: true` markieren

Gebaut mit [Tauri 2](https://tauri.app) – Rust im Hintergrund, Vue + TypeScript für das Overlay.

## Setup

Voraussetzungen: [Rust](https://rustup.rs), Node.js ≥ 20 und unter Ubuntu:

```bash
sudo apt install -y libwebkit2gtk-4.1-dev libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

```bash
npm install
npm run tauri dev
```

Nur am Design arbeiten (ohne Rust, mit Beispieldaten im Browser): `npm run dev` → http://localhost:1420

## Bedienung

Die App läuft im Hintergrund mit Tray-Icon. Standard-Hotkey: <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>/</kbd>
(macOS: <kbd>⌘</kbd><kbd>⌥</kbd><kbd>/</kbd>). Hotkey und Farben stehen in `config.yaml` im Config-Ordner
(Linux: `~/.config/dev.hubernicolas.shortcut-overlay/config.yaml`):

```yaml
hotkey: CommandOrControl+Alt+Slash
theme:
  primary: "#5eeaff"     # UI, gedrückte Tasten
  secondary: "#ff5ec8"   # Shortcuts mit learn: true
```

### Ubuntu / GNOME mit Wayland

Wayland erlaubt Apps weder globale Hotkeys noch das Abfragen des fokussierten Fensters. Deshalb:

1. **Fokus-Erkennung:** Beim ersten Start installiert die App eine kleine GNOME-Shell-Extension
   (`x-win@miniben90.org`). Danach **einmal ab- und wieder anmelden**.
2. **Hotkey:** *Einstellungen → Tastatur → Tastaturkürzel → Eigene Tastaturkürzel* → neues Kürzel
   mit dem Befehl `shortcut-overlay --toggle` (im Dev-Modus: Pfad zu `src-tauri/target/debug/shortcut-overlay`).

## Eigene Shortcuts

Der Ordner öffnet sich über das Tray-Menü oder „edit shortcuts“ im Overlay
(Linux: `~/.config/dev.hubernicolas.shortcut-overlay/shortcuts/`). Eine Datei mit
gleicher `id` wie ein eingebautes Profil ersetzt dieses; eine neue `id` fügt ein Programm hinzu.

```yaml
id: obsidian
name: Obsidian
match: [obsidian]                      # Programmname (steht im Overlay oben rechts bei "focus")
groups:
  - name: Notes
    shortcuts:
      - { keys: "Mod+O", action: "Quick switcher" }            # Mod = Ctrl bzw. ⌘ auf macOS
      - { keys: "Mod+P", action: "Command palette", learn: true }
      - { keys: "Mod+K Mod+S", action: "Zwei Schritte nacheinander" }
      - { keys: "Ctrl+Tab", mac: "Cmd+Alt+Right", action: "Pro OS überschreiben" }
      - { keys: "Super+E", action: "Nur auf manchen Systemen", os: [windows] }
```

Eingebaute Profile: [`src-tauri/shortcuts/`](src-tauri/shortcuts/).

## Release

Ein Tag `v*` pushen → GitHub Actions baut Installer für alle drei Systeme als Draft-Release.

## Struktur

```
src/                  Vue-Overlay (App.vue, KeyCombo.vue, keys.ts)
src-tauri/src/
  lib.rs              Fenster, Tray, Hotkey, --toggle
  focus.rs            Fokussiertes Programm erkennen (x-win)
  shortcuts.rs        YAML-Profile laden und zuordnen
src-tauri/shortcuts/  Eingebaute Profile
```
