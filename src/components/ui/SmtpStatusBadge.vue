<template>
  <div class="status-badge">
    <span class="dot" :class="running ? 'dot--on' : 'dot--off'" />
    <span>SMTP {{ running ? 'listening' : 'stopped' }}</span>
    <span class="port">:{{ port }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useConfigStore } from '@/stores/config'
import { storeToRefs } from 'pinia'

const configStore = useConfigStore()
const { config, smtpRunning } = storeToRefs(configStore)

const running = computed(() => smtpRunning.value)
const port = computed(() => config.value.smtp_port)
</script>

<style scoped>
.status-badge {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--text-muted);
}
.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}
.dot--on  { background: var(--success); }
.dot--off { background: var(--danger); }
.port { font-weight: 600; color: var(--text-secondary); }
</style>
