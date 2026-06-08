<template>
  <div
    class="mail-item"
    :class="{ selected: isSelected, unread: !mail.is_read }"
    @click="$emit('select', mail.id)"
  >
    <div class="item-row">
      <span v-if="!mail.is_read" class="unread-dot" />
      <span class="sender">{{ displayName }}</span>
      <span class="time">{{ formatDate(mail.received_at) }}</span>
    </div>
    <div class="subject">{{ mail.subject || '(no subject)' }}</div>
    <div class="meta">
      <span v-if="mail.has_html" class="tag tag--html">HTML</span>
      <span v-if="mail.attachment_count > 0" class="tag tag--files">{{ mail.attachment_count }} file</span>
      <span class="size">{{ formatSize(mail.size_bytes) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatDate, formatSize } from '@/lib/utils'
import type { MailSummary } from '@/types/mail'

const props = defineProps<{
  mail: MailSummary
  isSelected: boolean
}>()

defineEmits<{ select: [id: string] }>()

const displayName = computed(
  () => props.mail.from_name || props.mail.from_addr,
)
</script>

<style scoped>
.mail-item {
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.1s;
}
.mail-item:hover    { background: var(--bg-hover); }
.mail-item.selected { background: var(--bg-selected); }
.mail-item.unread .sender { color: var(--accent); }
.mail-item.unread .subject { color: var(--text-primary); font-weight: 500; }

.unread-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent);
  flex-shrink: 0;
}

.item-row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 8px;
}

.sender {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.time {
  font-size: 11px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.subject {
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-top: 2px;
}

.meta {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 4px;
}

.tag {
  font-size: 10px;
  font-weight: 600;
  padding: 1px 5px;
  border-radius: 3px;
}
.tag--html { background: #dbeafe; color: #1d4ed8; }
.tag--files { background: #dcfce7; color: #15803d; }

[data-theme="dark"] .tag--html { background: #1e3a5f; color: #93c5fd; }
[data-theme="dark"] .tag--files { background: #14532d; color: #86efac; }

.size {
  font-size: 11px;
  color: var(--text-muted);
  margin-left: auto;
}
</style>
