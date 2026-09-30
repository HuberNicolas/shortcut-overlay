<script setup lang="ts">
import { keyLabel, normKey } from "../keys";

defineProps<{ keys: string[][]; os: string; pressed: Set<string> }>();
</script>

<template>
  <span class="combo">
    <template v-for="(chord, i) in keys" :key="i">
      <span v-if="i > 0" class="then">then</span>
      <span class="chord">
        <kbd
          v-for="(k, j) in chord"
          :key="j"
          :class="{ wide: keyLabel(k, os).length > 2, down: pressed.has(normKey(k)) }"
        >{{ keyLabel(k, os) }}</kbd>
      </span>
    </template>
  </span>
</template>

<style scoped>
.combo {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  flex-shrink: 0;
}

.chord {
  display: inline-flex;
  gap: 3px;
}

.then {
  font-size: 9px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--dim);
}

kbd {
  font-family: var(--mono);
  font-size: 11px;
  font-weight: 600;
  min-width: 21px;
  height: 21px;
  padding: 0 5px;
  display: inline-grid;
  place-items: center;
  color: var(--text);
  background: var(--cap-bg);
  border: 1px solid var(--cap-edge);
  border-bottom-width: 2px;
  border-radius: 5px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  transition: border-color 0.15s, color 0.15s, box-shadow 0.15s, background 0.15s, transform 0.08s;
}

kbd.wide {
  font-size: 10px;
  letter-spacing: 0.02em;
}

/* Key currently held down on the keyboard */
kbd.down {
  color: var(--bg-solid);
  background: var(--accent);
  border-color: var(--accent);
  border-bottom-width: 1px;
  transform: translateY(1px);
  box-shadow:
    0 0 14px var(--accent),
    0 0 4px var(--accent);
  transition-duration: 0.04s;
}
</style>
