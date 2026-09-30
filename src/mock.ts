// Sample data for designing in a plain browser (`npm run dev`), without the Rust side.
import type { OverlayState, Profile } from "./types";

const k = (s: string) => s.split(" ").map((c) => c.split("+"));

const vscode: Profile = {
  id: "vscode",
  name: "Visual Studio Code",
  groups: [
    {
      name: "Essentials",
      shortcuts: [
        { keys: k("Ctrl+Shift+P"), action: "Command palette", learn: false },
        { keys: k("Ctrl+P"), action: "Quick open file", learn: false },
        { keys: k("Ctrl+K Ctrl+S"), action: "Keyboard shortcuts", learn: false },
        { keys: k("Ctrl+`"), action: "Toggle terminal", learn: false },
        { keys: k("Ctrl+B"), action: "Toggle sidebar", learn: false },
      ],
    },
    {
      name: "Editing",
      shortcuts: [
        { keys: k("Ctrl+D"), action: "Select next occurrence", learn: true },
        { keys: k("Ctrl+Shift+L"), action: "Select all occurrences", learn: false },
        { keys: k("Alt+Up"), action: "Move line up", learn: false },
        { keys: k("Ctrl+Shift+Alt+Down"), action: "Duplicate line down", learn: true },
        { keys: k("Ctrl+/"), action: "Toggle comment", learn: false },
        { keys: k("Ctrl+."), action: "Quick fix", learn: false },
      ],
    },
    {
      name: "Navigation",
      shortcuts: [
        { keys: k("F12"), action: "Go to definition", learn: false },
        { keys: k("F2"), action: "Rename symbol", learn: false },
        { keys: k("Ctrl+Shift+O"), action: "Go to symbol in file", learn: false },
        { keys: k("Ctrl+Alt+-"), action: "Navigate back", learn: false },
      ],
    },
  ],
};

const system: Profile = {
  id: "system",
  name: "System",
  groups: [
    {
      name: "Windows & apps",
      shortcuts: [
        { keys: k("Alt+Tab"), action: "Switch apps", learn: false },
        { keys: k("Super+Left"), action: "Tile window left", learn: false },
        { keys: k("Super+PageDown"), action: "Next workspace", learn: false },
      ],
    },
  ],
};

export const mockState: OverlayState = {
  app: { exec: "code", name: "code", title: "App.vue — shortcut-overlay" },
  profile: vscode,
  system,
  error: null,
  warnings: [],
  extension: "Ready",
  os: "linux",
  hotkey: "CommandOrControl+Alt+Slash",
  shortcuts_dir: "~/.config/dev.hubernicolas.shortcut-overlay/shortcuts",
};
