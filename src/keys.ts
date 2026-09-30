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

// ---- live key presses ------------------------------------------------------

/** Keys by physical position, used when Shift/Alt changed the character (e.g. Shift+/ → "?"). */
const BY_CODE: Record<string, string> = {
  Slash: "/",
  Period: ".",
  Comma: ",",
  Backquote: "`",
  Backslash: "\\",
  Minus: "-",
  Equal: "=",
  BracketLeft: "[",
  BracketRight: "]",
  Semicolon: ";",
  Quote: "'",
  Space: "Space",
};

const BY_KEY: Record<string, string> = {
  Control: "Ctrl",
  AltGraph: "Alt",
  ArrowUp: "Up",
  ArrowDown: "Down",
  ArrowLeft: "Left",
  ArrowRight: "Right",
  Escape: "Esc",
  PrintScreen: "Print",
  " ": "Space",
  "+": "Plus",
};

export const MODIFIERS = new Set(["Ctrl", "Shift", "Alt", "Cmd", "Super"]);

/** Maps a keyboard event to the key names used in the YAML profiles. */
export function eventKey(e: KeyboardEvent, os: string): string {
  if (e.key === "Meta" || e.key === "OS" || e.key === "Super") return os === "macos" ? "Cmd" : "Super";
  if (BY_KEY[e.key]) return BY_KEY[e.key];
  if (/^Key[A-Z]$/.test(e.code)) return /^[a-z]$/i.test(e.key) ? e.key.toUpperCase() : e.code.slice(3);
  if (/^Digit\d$/.test(e.code)) return e.code.slice(5);
  if (e.key.length === 1 && !e.shiftKey && !e.altKey) return e.key;
  return BY_CODE[e.code] ?? e.key;
}

/** Normalizes a key name from a profile so it can be compared with `eventKey`. */
export function normKey(key: string): string {
  if (key === "Escape") return "Esc";
  return key.length === 1 ? key.toUpperCase() : key;
}
