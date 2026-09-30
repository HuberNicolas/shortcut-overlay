// Turns key names from the YAML files into what's printed on a keycap.

const COMMON: Record<string, string> = {
  Up: "↑",
  Down: "↓",
  Left: "←",
  Right: "→",
  Plus: "+",
  Minus: "−",
  PageUp: "PgUp",
  PageDown: "PgDn",
  Print: "PrtSc",
  Delete: "Del",
  Escape: "Esc",
};

const MAC: Record<string, string> = {
  Cmd: "⌘",
  Alt: "⌥",
  Shift: "⇧",
  Ctrl: "⌃",
  Enter: "↩",
  Backspace: "⌫",
  Tab: "⇥",
  Esc: "⎋",
};

export function keyLabel(key: string, os: string): string {
  if (os === "macos" && MAC[key]) return MAC[key];
  if (key === "Super") return os === "windows" ? "Win" : "Super";
  if (key === "CommandOrControl") return os === "macos" ? "⌘" : "Ctrl";
  return COMMON[key] ?? key;
}

/** Formats a Tauri accelerator such as `CommandOrControl+Alt+Slash` for display. */
export function hotkeyLabel(hotkey: string, os: string): string {
  return hotkey
    .split("+")
    .map((k) => keyLabel(k === "Slash" ? "/" : k, os))
    .join(os === "macos" ? "" : "+");
}
