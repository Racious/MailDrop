<template>
  <div class="empty">
    <MailboxIcon :size="40" class="empty-icon" />
    <p class="empty-title">{{ title }}</p>
    <p v-if="description" class="empty-desc">{{ description }}</p>
  </div>
</template>

<script setup lang="ts">
import { MailboxIcon } from '@lucide/vue'

defineProps<{
  title: string
  description?: string
}>()
</script>

<style scoped>
.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 8px;
  color: var(--text-muted);
  text-align: center;
  padding: 24px;
}
.empty-icon { opacity: 0.4; }
.empty-title { font-size: 14px; font-weight: 500; margin: 0; }
.empty-desc  { font-size: 12px; margin: 0; }
</style>
