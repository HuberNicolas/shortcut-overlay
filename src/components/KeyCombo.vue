<script setup lang="ts">
import { keyLabel } from "../keys";

defineProps<{ keys: string[][]; os: string }>();
</script>

<template>
  <span class="combo">
    <template v-for="(chord, i) in keys" :key="i">
      <span v-if="i > 0" class="then">then</span>
      <span class="chord">
        <kbd v-for="(k, j) in chord" :key="j" :class="{ wide: keyLabel(k, os).length > 2 }">{{ keyLabel(k, os) }}</kbd>
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
  transition: border-color 0.15s, color 0.15s, box-shadow 0.15s;
}

kbd.wide {
  font-size: 10px;
  letter-spacing: 0.02em;
}
</style>
