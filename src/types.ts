// Mirrors the structs in src-tauri/src/lib.rs and shortcuts.rs.

export interface Shortcut {
  keys: string[][];
  action: string;
  learn: boolean;
}

export interface Group {
  name: string;
  shortcuts: Shortcut[];
}

export interface Profile {
  id: string;
  name: string;
  groups: Group[];
}

export interface ActiveApp {
  exec: string;
  name: string;
  title: string;
}

export type ExtensionStatus = "NotNeeded" | "Ready" | "NeedsRelogin" | { Error: string };

export interface OverlayState {
  app: ActiveApp | null;
  profile: Profile | null;
  system: Profile | null;
  error: string | null;
  warnings: string[];
  extension: ExtensionStatus;
  os: "linux" | "macos" | "windows";
  hotkey: string;
  shortcuts_dir: string;
}
