<template>
  <div class="preview-header">
    <div class="header-main">
      <h2 class="subject">{{ mail.subject || '(no subject)' }}</h2>
      <button class="delete-btn" title="Delete mail" @click="onDelete">
        <Trash2Icon :size="14" />
      </button>
    </div>
    <div class="header-meta">
      <span class="from">
        <strong>{{ mail.from_name || mail.from_addr }}</strong>
        <span v-if="mail.from_name" class="addr">&lt;{{ mail.from_addr }}&gt;</span>
      </span>
      <span class="time">{{ formatDate(mail.received_at) }}</span>
    </div>
    <div class="header-to">
      <span class="label">To:</span>
      {{ mail.to_addrs.join(', ') }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { Trash2Icon } from '@lucide/vue'
import { formatDate } from '@/lib/utils'
import type { Mail } from '@/types/mail'
import { useMailStore } from '@/stores/mail'

const props = defineProps<{ mail: Mail }>()
const mailStore = useMailStore()

async function onDelete() {
  await mailStore.removeMail(props.mail.id)
}
</script>

<style scoped>
.preview-header {
  padding: 14px 16px 10px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-surface);
}

.header-main {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.subject {
  flex: 1;
  margin: 0 0 6px;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.3;
}

.delete-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.15s;
}
.delete-btn:hover {
  background: var(--danger);
  border-color: var(--danger);
  color: #fff;
}

.header-meta {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.from { display: flex; gap: 4px; align-items: baseline; flex-wrap: wrap; }
.addr { color: var(--text-muted); }
.time { flex-shrink: 0; color: var(--text-muted); }

.header-to {
  font-size: 12px;
  color: var(--text-secondary);
}
.label { color: var(--text-muted); margin-right: 4px; }
</style>
