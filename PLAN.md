# Plan: Shortcut Overlay

Ein kleines Desktop-Tool, das per Hotkey ein halbtransparentes Overlay mit den
wichtigsten Tastenkürzeln des **aktuell fokussierten Programms** einblendet.
Läuft auf **Ubuntu, macOS und Windows**.

## 1. Sprachwahl

| Option | Pro | Contra |
|---|---|---|
| **Python + PySide6 (Qt)** ✅ | Kennst du am besten; Qt kann rahmenlose, transparente, Always-on-top-Fenster auf allen 3 OS; mit PyInstaller als einzelne App paketierbar | Binary ~50–80 MB; Fokus-Erkennung pro OS selbst schreiben |
| Electron + TypeScript | Kennst du auch; `get-windows` + `globalShortcut` fertig vorhanden | Sehr schwer (~150 MB, viel RAM) für ein kleines Overlay |
| Tauri (Rust + TS) | Klein und schnell | Rust neu für dich; Fokus-Erkennung trotzdem selbst |

**Entscheidung: Python 3.12 + PySide6.** Die Plattform-Unterschiede stecken in
einem kleinen, austauschbaren Modul – der Rest ist identischer Code.

## 2. Die zwei plattformspezifischen Knackpunkte

### a) Welches Programm hat gerade den Fokus?

| OS | Umsetzung |
|---|---|
| Windows | `ctypes` → `GetForegroundWindow` + `GetWindowThreadProcessId` → Prozessname (via `psutil`) |
| macOS | `pyobjc` → `NSWorkspace.frontmostApplication()` (Bundle-ID, z. B. `com.microsoft.VSCode`) |
| Linux X11 | `xprop` / `python-xlib` → `_NET_ACTIVE_WINDOW` → `WM_CLASS` |
| Linux **Wayland (GNOME)** | Wayland erlaubt das nicht direkt. Lösung: GNOME-Extension **„Window Calls“** installieren, die den Fokus per D-Bus liefert |

⚠️ Dein Ubuntu läuft auf **Wayland/GNOME** – den Fall müssen wir also sicher abdecken.

### b) Globaler Hotkey zum Ein-/Ausblenden

Globale Hotkeys sind unter Wayland ebenfalls gesperrt. Deshalb ein Ansatz, der
überall gleich funktioniert:

- Die App läuft im Hintergrund (Tray-Icon) und lauscht auf einem lokalen Socket.
- `shortcut-overlay --toggle` schickt nur ein „Toggle“ an die laufende Instanz.
- **Linux:** in den GNOME-Einstellungen einen eigenen Shortcut (z. B. `Super+/`) auf diesen Befehl legen.
- **Windows/macOS:** zusätzlich direkt in der App per `pynput` registrieren (macOS fragt einmalig nach Bedienungshilfen-Berechtigung).

Wichtig: Das Fokus-Programm wird **beim Drücken des Hotkeys** abgefragt, bevor
das Overlay selbst den Fokus bekommt.

## 3. Shortcut-Daten

Eine YAML-Datei pro Programm, mitgeliefert unter `shortcuts/`, überschreibbar im
User-Config-Ordner (`platformdirs`):

```yaml
# shortcuts/vscode.yaml
name: Visual Studio Code
match:              # woran wir das Programm erkennen
  windows: [Code.exe]
  macos: [com.microsoft.VSCode]
  linux: [code, Code]
shortcuts:
  - group: Allgemein
    items:
      - keys: { default: "Ctrl+Shift+P", macos: "Cmd+Shift+P" }
        action: Befehlspalette
        learning: false   # später: true = „will ich lernen“ → hervorgehoben
```

Default-Set für den Start: VS Code, Firefox/Chrome, Terminal, Datei-Explorer
(Nautilus/Finder/Explorer), JetBrains-IDEs, Slack, plus ein **Fallback**
„System“ (Fenster wechseln, Screenshot, …) wenn nichts passt.

## 4. Projektstruktur

```
shortcut-overlay/
├── pyproject.toml
├── src/shortcut_overlay/
│   ├── __main__.py        # CLI: start / --toggle
│   ├── app.py             # Qt-App, Tray, Socket-Server
│   ├── overlay.py         # Overlay-Fenster (transparent, zentriert)
│   ├── shortcuts.py       # YAML laden, Programm matchen
│   └── focus/             # get_active_app() pro OS
│       ├── windows.py
│       ├── macos.py
│       └── linux.py       # X11 + Wayland/GNOME
├── shortcuts/*.yaml       # mitgelieferte Defaults
└── tests/
```

## 5. Umsetzung in Schritten

1. **Grundgerüst:** `pyproject.toml` (uv), PySide6, Overlay-Fenster mit Dummy-Daten.
2. **YAML-Format + Default-Shortcuts** für ~6 Programme + Fallback.
3. **Fokus-Erkennung** für Linux (Wayland + X11) – zuerst auf deinem Rechner testen.
4. **Toggle-Mechanismus:** Hintergrundinstanz + `--toggle` + GNOME-Shortcut.
5. **Windows + macOS** Fokus-Erkennung und Hotkey.
6. **Packaging:** GitHub Actions baut mit PyInstaller Releases für alle 3 OS.
7. **Später:** „Lernmodus“ – eigene Shortcuts markieren/hinzufügen, hervorheben, ggf. kleiner Editor in der App.

## 6. Offene Fragen an dich

- Welcher Hotkey soll das Overlay öffnen? (Vorschlag: `Super+/` bzw. `Cmd+/`)
- Overlay nur **solange gedrückt** (wie bei macOS „CheatSheet“) oder **Toggle** (einmal auf, einmal zu)?
- Welche Programme sind dir für das Default-Set am wichtigsten?
