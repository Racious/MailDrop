<template>
  <div class="tabs">
    <button
      v-for="tab in availableTabs"
      :key="tab"
      class="tab"
      :class="{ active: modelValue === tab }"
      @click="$emit('update:modelValue', tab)"
    >
      {{ tab }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: 'HTML' | 'Text' | 'Raw'
  hasHtml: boolean
  hasText: boolean
}>()

defineEmits<{ 'update:modelValue': [v: 'HTML' | 'Text' | 'Raw'] }>()

const availableTabs = computed(() => {
  const tabs: ('HTML' | 'Text' | 'Raw')[] = []
  if (props.hasHtml) tabs.push('HTML')
  if (props.hasText) tabs.push('Text')
  tabs.push('Raw')
  return tabs
})
</script>

<style scoped>
.tabs {
  display: flex;
  gap: 2px;
  padding: 0 16px;
  border-bottom: 1px solid var(--border);
}

.tab {
  padding: 8px 14px;
  font-size: 12px;
  font-weight: 500;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  margin-bottom: -1px;
  transition: all 0.15s;
}
.tab:hover  { color: var(--text-primary); }
.tab.active { color: var(--accent); border-bottom-color: var(--accent); }
</style>
