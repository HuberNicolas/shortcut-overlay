<script setup lang="ts">
import { PRESETS, hexToHue, hueToHex } from "../theme";
import type { Theme } from "../types";

const props = defineProps<{ theme: Theme }>();
const emit = defineEmits<{ change: [Theme] }>();

function setHue(slot: keyof Theme, hue: number) {
  emit("change", { ...props.theme, [slot]: hueToHex(hue) });
}

const isActive = (t: Theme) => t.primary === props.theme.primary && t.secondary === props.theme.secondary;
</script>

<template>
  <div class="picker" @keydown.stop>
    <div class="label">// theme</div>
    <div class="presets">
      <button
        v-for="p in PRESETS"
        :key="p.name"
        :class="{ on: isActive(p.theme) }"
        :title="p.name"
        @click="emit('change', p.theme)"
      >
        <i :style="{ background: p.theme.primary }" /><i :style="{ background: p.theme.secondary }" />
        <span>{{ p.name }}</span>
      </button>
    </div>
    <label v-for="slot in ['primary', 'secondary'] as const" :key="slot" class="slider">
      <span class="name">{{ slot }}</span>
      <input
        type="range"
        min="0"
        max="359"
        :value="hexToHue(theme[slot])"
        @input="setHue(slot, +($event.target as HTMLInputElement).value)"
      />
      <span class="hex" :style="{ color: theme[slot] }">{{ theme[slot] }}</span>
    </label>
  </div>
</template>

<style scoped>
.picker {
  position: absolute;
  right: 20px;
  bottom: 48px;
  z-index: 10;
  width: 300px;
  padding: 12px 14px;
  background: var(--bg-solid);
  border: 1px solid var(--panel-edge);
  border-radius: 10px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.6), 0 0 24px var(--accent-soft);
  animation: pop 0.18s ease-out;
}
@keyframes pop {
  from { opacity: 0; transform: translateY(6px); }
}
.label {
  margin-bottom: 8px;
  font-size: 10px;
  letter-spacing: 0.2em;
  text-transform: uppercase;
  color: var(--accent);
}
.presets {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 5px;
  margin-bottom: 10px;
}
.presets button {
  display: flex;
  align-items: center;
  gap: 3px;
  padding: 5px 6px;
  font: inherit;
  font-size: 10px;
  color: var(--muted);
  background: transparent;
  border: 1px solid var(--dim);
  border-radius: 5px;
  cursor: pointer;
}
.presets button:hover { color: var(--text); }
.presets button.on { color: var(--text); border-color: var(--accent); }
.presets i { width: 9px; height: 9px; border-radius: 2px; }
.presets span { margin-left: 3px; }
.slider {
  display: grid;
  grid-template-columns: 70px 1fr 62px;
  align-items: center;
  gap: 8px;
  font-size: 10px;
  color: var(--muted);
  margin-top: 4px;
}
.hex { text-align: right; }
input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  height: 8px;
  border-radius: 4px;
  background: linear-gradient(90deg, #ff4f4f, #ffd84f, #4fff7a, #4ff0ff, #4f6bff, #d84fff, #ff4f4f);
  cursor: pointer;
}
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 12px;
  height: 14px;
  border-radius: 3px;
  background: var(--text);
  border: 2px solid var(--bg-solid);
}
</style>
