# Your shortcuts

Drop `.yaml` files into this folder. They are re-read every time the overlay opens.

- A file with the same `id` as a built-in profile **replaces** it
  (built-ins: system, vscode, firefox, chromium, terminal, files, jetbrains, slack).
- A file with a new `id` adds a profile for another app.
- `learn: true` highlights a shortcut you want to practice.

```yaml
id: obsidian
name: Obsidian
match: [obsidian]          # executable or app name (case-insensitive), shown in the overlay header
groups:
  - name: Notes
    shortcuts:
      - { keys: "Mod+O", action: "Quick switcher" }          # Mod = Ctrl, or Cmd on macOS
      - { keys: "Mod+P", action: "Command palette", learn: true }
      - { keys: "Mod+K Mod+S", action: "Two-step chord" }    # space = sequence
      - { keys: "Ctrl+Tab", mac: "Cmd+Alt+Right", action: "Per-OS override" }
      - { keys: "Super+E", action: "Only on some systems", os: [windows] }
```

Special key names: `Mod`, `Ctrl`, `Alt`, `Shift`, `Super`, `Cmd`, `Plus`, `Enter`, `Space`, `Up`, `Tab`, …

The hotkey can be changed in `../config.yaml`: `hotkey: "CommandOrControl+Alt+Slash"`.
