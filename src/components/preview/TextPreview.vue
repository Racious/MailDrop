<template>
  <div class="text-preview">
    <pre class="text-body">{{ text }}</pre>
  </div>
</template>

<script setup lang="ts">
defineProps<{ text: string }>()
</script>

<style scoped>
.text-preview {
  flex: 1;
  overflow: auto;
  padding: 16px;
  background: var(--bg-base);
  height: 100%;
}
.text-body {
  margin: 0;
  font-family: 'Cascadia Code', 'Fira Mono', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  color: var(--text-primary);
}
</style>
