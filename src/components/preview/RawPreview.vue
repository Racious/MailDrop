<template>
  <div class="raw-preview">
    <pre class="raw-body">{{ raw }}</pre>
  </div>
</template>

<script setup lang="ts">
defineProps<{ raw: string }>()
</script>

<style scoped>
.raw-preview {
  flex: 1;
  overflow: auto;
  padding: 16px;
  background: var(--bg-base);
  height: 100%;
}
.raw-body {
  margin: 0;
  font-family: 'Cascadia Code', 'Fira Mono', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--text-secondary);
}
</style>
