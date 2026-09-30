<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import KeyCombo from "./components/KeyCombo.vue";
import ThemePicker from "./components/ThemePicker.vue";
import { MODIFIERS, eventKey, hotkeyLabel, keyLabel, normKey } from "./keys";
import { PRESETS, applyTheme } from "./theme";
import type { OverlayState, Profile, Shortcut, Theme } from "./types";

const inTauri = "__TAURI_INTERNALS__" in window;

const state = ref<OverlayState | null>(null);
const tab = ref<"app" | "system">("app");
const filter = ref("");
// Bumped on every show to replay the entry animation.
const showId = ref(0);
// Keys currently held down, as YAML key names ("Ctrl", "Shift", "P", …).
const pressed = ref(new Set<string>());
const theme = ref<Theme>(PRESETS[0].theme);
const showTheme = ref(false);

const active = computed<Profile | null>(() => {
  const s = state.value;
  if (!s) return null;
  return tab.value === "app" && s.profile ? s.profile : s.system;
});

const groups = computed(() => {
  const q = filter.value.trim().toLowerCase();
  const all = active.value?.groups ?? [];
  if (!q) return all;
  return all
    .map((g) => ({
      ...g,
      shortcuts: g.shortcuts.filter(
        (s) => s.action.toLowerCase().includes(q) || s.keys.flat().join(" ").toLowerCase().includes(q),
      ),
    }))
    .filter((g) => g.shortcuts.length > 0);
});

const total = computed(() => groups.value.reduce((n, g) => n + g.shortcuts.length, 0));
const learnCount = computed(() => groups.value.reduce((n, g) => n + g.shortcuts.filter((s) => s.learn).length, 0));

const notice = computed(() => {
  const s = state.value;
  if (!s) return null;
  if (s.extension === "NeedsRelogin")
    return "GNOME helper extension installed — log out and back in once to enable app detection.";
  if (typeof s.extension === "object") return `GNOME extension: ${s.extension.Error}`;
  if (s.error) return `Could not detect focused app: ${s.error}`;
  if (s.warnings.length) return `Skipped: ${s.warnings.join(" · ")}`;
  return null;
});

function applyState(s: OverlayState) {
  state.value = s;
  tab.value = s.profile ? "app" : "system";
  filter.value = "";
  pressed.value.clear();
  showTheme.value = false;
  theme.value = s.theme;
  applyTheme(s.theme);
  showId.value++;
}

let saveTimer: number | undefined;

function setTheme(t: Theme) {
  theme.value = t;
  applyTheme(t);
  // Slider drags fire a lot of events; only persist once it settles.
  clearTimeout(saveTimer);
  saveTimer = window.setTimeout(() => inTauri && invoke("set_theme", { theme: t }), 300);
}

/** True if exactly the keys of one step of this shortcut are held down. */
function isHit(s: Shortcut) {
  const p = pressed.value;
  return s.keys.some((chord) => chord.length === p.size && chord.every((k) => p.has(normKey(k))));
}

const held = computed(() => [...pressed.value]);

// Bring a fully matched shortcut into view.
watch(
  () => held.value.join("+"),
  async () => {
    await nextTick();
    document.querySelector("li.hit")?.scrollIntoView({ block: "nearest", behavior: "smooth" });
  },
);

function hide() {
  if (inTauri) invoke("hide_overlay");
}

function openFolder() {
  if (inTauri) invoke("open_shortcuts_dir");
}

function onKey(e: KeyboardEvent) {
  pressed.value.add(eventKey(e, state.value?.os ?? ""));
  const chord = e.ctrlKey || e.metaKey || e.altKey;

  if (e.key === "Escape") {
    if (showTheme.value) showTheme.value = false;
    else if (filter.value) filter.value = "";
    else hide();
  } else if (e.key === "Tab" && !chord) {
    if (state.value?.profile) tab.value = tab.value === "app" ? "system" : "app";
  } else if (e.key === "Backspace" && !chord) {
    filter.value = filter.value.slice(0, -1);
  } else if (e.key.length === 1 && e.key !== " " && !chord) {
    filter.value += e.key;
  } else if (!chord) {
    return;
  }
  // Swallow combos so the webview doesn't act on them (reload, find, zoom, …).
  e.preventDefault();
}

function onKeyUp(e: KeyboardEvent) {
  const k = eventKey(e, state.value?.os ?? "");
  pressed.value.delete(k);
  // macOS drops keyup events of other keys while Cmd is held.
  if (MODIFIERS.has(k)) for (const p of pressed.value) if (!MODIFIERS.has(p)) pressed.value.delete(p);
}

function clearPressed() {
  pressed.value.clear();
}

let unlisten: UnlistenFn | undefined;

onMounted(async () => {
  window.addEventListener("keydown", onKey);
  window.addEventListener("keyup", onKeyUp);
  window.addEventListener("blur", clearPressed);
  if (inTauri) {
    unlisten = await listen<OverlayState>("overlay://show", (e) => applyState(e.payload));
    const current = await invoke<OverlayState | null>("current_state");
    if (current) applyState(current);
  } else {
    // Plain browser (`npm run dev`): render sample data to work on the design.
    // For screenshots: `?hold=Ctrl+Shift+P` shows keys as held down, `?still` skips animations.
    const { mockState } = await import("./mock");
    const params = new URLSearchParams(location.search);
    document.body.style.background = "#05070b";
    document.body.classList.toggle("still", params.has("still"));
    await nextTick();
    applyState(mockState);
    params.get("hold")?.split(" ").forEach((k) => pressed.value.add(k));
  }
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKey);
  window.removeEventListener("keyup", onKeyUp);
  window.removeEventListener("blur", clearPressed);
  unlisten?.();
});
</script>

<template>
  <main v-if="state" :key="showId" class="panel">
    <i class="corner tl" /><i class="corner tr" /><i class="corner bl" /><i class="corner br" />
    <div class="scan" />

    <header>
      <div class="brand">
        <span class="glyph">◢</span> SHORTCUT<span class="slash">//</span>OVERLAY
        <span class="ver">v0.1</span>
      </div>
      <div class="meta">
        <span class="chip"><b>os</b>{{ state.os }}</span>
        <span class="chip" :title="state.app?.title">
          <span class="dot" :class="{ off: !state.profile }" />
          <b>focus</b>{{ state.app?.exec || "unknown" }}
        </span>
      </div>
    </header>

    <section class="title">
      <h1>
        <span class="prompt">&gt;</span>
        {{ active?.name ?? "No shortcuts" }}
      </h1>
      <nav class="tabs">
        <button v-if="state.profile" :class="{ on: tab === 'app' }" @click="tab = 'app'">
          {{ state.profile.name }}
        </button>
        <button :class="{ on: tab === 'system' }" @click="tab = 'system'">System</button>
      </nav>
    </section>

    <div v-if="!state.profile" class="nomatch">
      No profile for <code>{{ state.app?.exec || "this app" }}</code> — showing system shortcuts.
      <a @click="openFolder">Add your own →</a>
    </div>

    <div class="filter" :class="{ active: filter }">
      <span class="prompt">filter:</span>
      <span class="query">{{ filter }}</span><span class="caret" />
      <span v-if="held.length" class="held">
        <template v-for="(k, i) in held" :key="k"><span v-if="i">+</span><b>{{ keyLabel(k, state.os) }}</b></template>
      </span>
      <span class="count">{{ String(total).padStart(2, "0") }} hits</span>
      <span v-if="learnCount" class="count learn">◆ {{ learnCount }} to learn</span>
    </div>

    <div class="grid">
      <article
        v-for="(g, gi) in groups"
        :key="active?.id + g.name"
        class="group"
        :style="{ '--d': gi * 60 + 'ms' }"
      >
        <h2>
          <span>// {{ g.name }}</span>
          <span class="n">{{ String(g.shortcuts.length).padStart(2, "0") }}</span>
        </h2>
        <ul>
          <li
            v-for="(s, si) in g.shortcuts"
            :key="si"
            :class="{ learn: s.learn, hit: isHit(s) }"
            :style="{ '--d': gi * 60 + si * 22 + 'ms' }"
          >
            <span class="action">{{ s.action }}</span>
            <span class="leader" />
            <KeyCombo :keys="s.keys" :os="state.os" :pressed="pressed" />
          </li>
        </ul>
      </article>
      <p v-if="!groups.length" class="empty">∅ no match for “{{ filter }}”</p>
    </div>

    <footer>
      <span><kbd>esc</kbd> close</span>
      <span v-if="state.profile"><kbd>tab</kbd> app ⇄ system</span>
      <span><kbd>a–z</kbd> filter</span>
      <span><kbd>{{ hotkeyLabel(state.hotkey, state.os) }}</kbd> toggle</span>
      <a class="folder" @click="openFolder">⌁ edit shortcuts</a>
      <a :class="{ on: showTheme }" @click="showTheme = !showTheme">◐ theme</a>
    </footer>
    <ThemePicker v-if="showTheme" :theme="theme" @change="setTheme" />
    <div v-if="notice" class="notice">⚠ {{ notice }}</div>
  </main>
</template>

<style scoped>
.panel {
  position: relative;
  width: calc(100% - 24px);
  max-height: calc(100% - 24px);
  display: flex;
  flex-direction: column;
  padding: 22px 28px 16px;
  border: 1px solid var(--panel-edge);
  border-radius: 14px;
  background:
    radial-gradient(ellipse 60% 40% at 15% 0%, color-mix(in srgb, var(--accent) 10%, transparent), transparent 70%),
    radial-gradient(ellipse 50% 40% at 100% 100%, color-mix(in srgb, var(--learn) 7%, transparent), transparent 70%),
    linear-gradient(var(--grid) 1px, transparent 1px) 0 0 / 100% 28px,
    linear-gradient(90deg, var(--grid) 1px, transparent 1px) 0 0 / 28px 100%,
    var(--bg);
  box-shadow:
    0 0 0 1px rgba(0, 0, 0, 0.6),
    0 0 40px color-mix(in srgb, var(--accent) 8%, transparent),
    inset 0 0 60px color-mix(in srgb, var(--accent) 4%, transparent);
  overflow: hidden;
  animation: boot 0.28s cubic-bezier(0.2, 0.9, 0.3, 1.2);
}

@keyframes boot {
  from {
    opacity: 0;
    transform: scale(0.97) translateY(6px);
    filter: blur(4px);
  }
}

/* HUD corner brackets */
.corner {
  position: absolute;
  width: 14px;
  height: 14px;
  border: 2px solid var(--accent);
  opacity: 0.85;
}
.tl { top: 8px; left: 8px; border-right: 0; border-bottom: 0; }
.tr { top: 8px; right: 8px; border-left: 0; border-bottom: 0; }
.bl { bottom: 8px; left: 8px; border-right: 0; border-top: 0; }
.br { bottom: 8px; right: 8px; border-left: 0; border-top: 0; }

/* One slow scanline sweeping down */
.scan {
  position: absolute;
  inset: 0;
  pointer-events: none;
  background:
    repeating-linear-gradient(0deg, rgba(255, 255, 255, 0.018) 0 1px, transparent 1px 3px),
    linear-gradient(180deg, transparent 0%, color-mix(in srgb, var(--accent) 6%, transparent) 50%, transparent 100%) 0 -100% / 100% 30% no-repeat;
  animation: sweep 6s linear infinite;
}
@keyframes sweep {
  to { background-position: 0 0, 0 400%; }
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 10px;
  letter-spacing: 0.22em;
  color: var(--muted);
}
.brand .glyph, .brand .slash { color: var(--accent); }
.brand .ver { margin-left: 10px; color: var(--dim); }
.meta { display: flex; gap: 8px; }
.chip {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  padding: 3px 9px;
  border: 1px solid var(--dim);
  border-radius: 99px;
  letter-spacing: 0.06em;
  color: var(--text);
  max-width: 280px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.chip b { color: var(--muted); font-weight: 400; }
.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--ok);
  box-shadow: 0 0 8px var(--ok);
  animation: pulse 1.6s ease-in-out infinite;
}
.dot.off { background: var(--warn); box-shadow: 0 0 8px var(--warn); }
@keyframes pulse { 50% { opacity: 0.35; } }

.title {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 20px;
  margin: 18px 0 12px;
}
h1 {
  margin: 0;
  font-family: var(--display);
  font-size: 34px;
  font-weight: 600;
  letter-spacing: -0.01em;
  line-height: 1;
  text-shadow: 0 0 24px color-mix(in srgb, var(--accent) 25%, transparent);
}
h1 .prompt { color: var(--accent); margin-right: 4px; }

.tabs { display: flex; gap: 4px; }
.tabs button {
  font: inherit;
  font-size: 11px;
  padding: 5px 12px;
  color: var(--muted);
  background: transparent;
  border: 1px solid var(--dim);
  border-radius: 6px;
  cursor: pointer;
}
.tabs button.on {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-soft);
  box-shadow: 0 0 12px color-mix(in srgb, var(--accent) 15%, transparent);
}

.nomatch {
  margin-bottom: 10px;
  padding: 8px 12px;
  font-size: 12px;
  color: var(--warn);
  border-left: 2px solid var(--warn);
  background: rgba(255, 204, 102, 0.06);
}
.nomatch code { color: var(--text); }

a {
  color: var(--accent);
  cursor: pointer;
  margin-left: 6px;
}
a:hover { text-decoration: underline; }

.filter {
  display: flex;
  align-items: center;
  padding: 7px 0 10px;
  border-bottom: 1px dashed var(--dim);
  color: var(--muted);
  font-size: 12px;
}
.filter .prompt { margin-right: 8px; }
.filter .query { color: var(--accent); }
.filter.active { border-bottom-color: var(--accent); }
.caret {
  width: 7px;
  height: 14px;
  margin-left: 1px;
  background: var(--accent);
  animation: blink 1s steps(1) infinite;
}
@keyframes blink { 50% { opacity: 0; } }
.count { margin-left: auto; color: var(--dim); }
.count.learn { margin-left: 16px; color: var(--learn); }

.grid {
  flex: 0 1 auto;
  min-height: 0;
  margin-top: 16px;
  columns: 3 280px;
  column-gap: 30px;
  overflow-y: auto;
  scrollbar-width: none;
}
.grid::-webkit-scrollbar { display: none; }

.group {
  break-inside: avoid;
  margin-bottom: 20px;
  animation: rise 0.35s var(--d) both ease-out;
}
h2 {
  display: flex;
  justify-content: space-between;
  margin: 0 0 8px;
  padding-bottom: 5px;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.2em;
  text-transform: uppercase;
  color: var(--accent);
  border-bottom: 1px solid var(--accent-soft);
}
h2 .n { color: var(--dim); }

ul { list-style: none; margin: 0; padding: 0; }
li {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 6px;
  margin: 0 -6px;
  border-radius: 5px;
  animation: rise 0.3s var(--d) both ease-out;
}
li:hover { background: var(--accent-soft); }
li:hover :deep(kbd) { border-color: var(--accent); color: var(--accent); box-shadow: 0 0 10px color-mix(in srgb, var(--accent) 25%, transparent); }
.action { min-width: 0; line-height: 1.35; }
.leader {
  flex: 1;
  min-width: 12px;
  height: 1px;
  align-self: center;
  background: repeating-linear-gradient(90deg, var(--dim) 0 2px, transparent 2px 5px);
  opacity: 0.6;
}

li.learn { background: var(--learn-soft); }
li.learn .action::before { content: "◆ "; color: var(--learn); }
li.learn :deep(kbd) { border-color: var(--learn); color: var(--learn); box-shadow: 0 0 10px color-mix(in srgb, var(--learn) 25%, transparent); }
/* Pressed keys win over hover/learn styling; in learn rows they glow in the secondary color. */
li :deep(kbd.down) { color: var(--bg-solid); }
li.learn :deep(kbd.down) {
  color: var(--bg-solid);
  background: var(--learn);
  border-color: var(--learn);
  box-shadow: 0 0 14px var(--learn), 0 0 4px var(--learn);
}

@keyframes rise {
  from { opacity: 0; transform: translateY(4px); }
}

.empty { color: var(--muted); }

footer {
  display: flex;
  align-items: center;
  gap: 18px;
  padding-top: 12px;
  border-top: 1px solid var(--accent-soft);
  font-size: 10px;
  color: var(--muted);
  letter-spacing: 0.05em;
}
footer kbd {
  font-family: var(--mono);
  font-size: 10px;
  padding: 1px 6px;
  margin-right: 4px;
  color: var(--text);
  border: 1px solid var(--dim);
  border-radius: 4px;
}
footer .folder { margin-left: auto; }
footer a.on { text-decoration: underline; }

.held {
  margin-left: 16px;
  padding: 1px 8px;
  font-size: 11px;
  color: var(--accent);
  border: 1px solid var(--accent);
  border-radius: 4px;
  box-shadow: 0 0 10px var(--accent-soft);
}
.held b { font-weight: 600; }
.held span { margin: 0 3px; color: var(--muted); }

/* Exactly this shortcut is being held */
li.hit {
  background: color-mix(in srgb, var(--accent) 20%, transparent);
  box-shadow: inset 2px 0 0 var(--accent), 0 0 18px color-mix(in srgb, var(--accent) 20%, transparent);
}
li.hit .action { color: var(--accent); }

.notice {
  margin-top: 8px;
  font-size: 11px;
  color: var(--warn);
}
</style>
