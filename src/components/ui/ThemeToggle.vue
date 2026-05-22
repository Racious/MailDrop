<template>
  <button class="toggle" :title="`Switch to ${isDark ? 'light' : 'dark'} mode`" @click="toggle">
    <SunIcon v-if="isDark" :size="15" />
    <MoonIcon v-else :size="15" />
  </button>
</template>

<script setup lang="ts">
import { MoonIcon, SunIcon } from '@lucide/vue'
import { useTheme } from '@/composables/useTheme'

const { isDark, toggle } = useTheme()
</script>

<style scoped>
.toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s;
}
.toggle:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
</style>
