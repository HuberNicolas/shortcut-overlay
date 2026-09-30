import type { Theme } from "./types";

export const PRESETS: { name: string; theme: Theme }[] = [
  { name: "cyber", theme: { primary: "#5eeaff", secondary: "#ff5ec8" } },
  { name: "matrix", theme: { primary: "#4dff88", secondary: "#ffd23f" } },
  { name: "synthwave", theme: { primary: "#ff4fd8", secondary: "#8a7dff" } },
  { name: "amber", theme: { primary: "#ffb000", secondary: "#ff5a36" } },
  { name: "ice", theme: { primary: "#7fb8ff", secondary: "#e6f0ff" } },
  { name: "toxic", theme: { primary: "#c6ff3d", secondary: "#ff3d7f" } },
];

export function applyTheme(t: Theme) {
  const root = document.documentElement.style;
  root.setProperty("--accent", t.primary);
  root.setProperty("--learn", t.secondary);
}

/** Hue (0–360) of a `#rrggbb` color. */
export function hexToHue(hex: string): number {
  const [r, g, b] = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16) / 255);
  const max = Math.max(r, g, b);
  const d = max - Math.min(r, g, b);
  if (d === 0) return 0;
  const h = max === r ? ((g - b) / d) % 6 : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  return Math.round((h * 60 + 360) % 360);
}

/** Bright neon color for a hue, as `#rrggbb`. */
export function hueToHex(h: number, s = 1, l = 0.62): string {
  const f = (n: number) => {
    const k = (n + h / 30) % 12;
    const c = l - s * Math.min(l, 1 - l) * Math.max(-1, Math.min(k - 3, 9 - k, 1));
    return Math.round(c * 255)
      .toString(16)
      .padStart(2, "0");
  };
  return `#${f(0)}${f(8)}${f(4)}`;
}
