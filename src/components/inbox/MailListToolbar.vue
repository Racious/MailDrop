<template>
  <div class="toolbar">
    <div class="search-wrap">
      <SearchIcon :size="13" class="search-icon" />
      <input
        v-model="query"
        class="search"
        placeholder="Search..."
        type="search"
        @input="emitFilters"
      />
    </div>
    <select v-model="field" class="field-select" title="Search field" @change="emitFilters">
      <option value="all">All</option>
      <option value="from">From</option>
      <option value="to">To</option>
      <option value="subject">Subject</option>
      <option value="body">Body</option>
      <option value="attachments">Files</option>
    </select>
    <button
      class="filter-btn"
      :class="{ active: unreadOnly }"
      title="Unread only"
      type="button"
      @click="toggleUnread"
    >
      Unread
    </button>
    <button
      class="filter-btn"
      :class="{ active: hasAttachments }"
      title="Has attachments"
      type="button"
      @click="toggleAttachments"
    >
      Files
    </button>
    <button class="clear-btn" title="Clear all mails" @click="onClear">
      <Trash2Icon :size="14" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { SearchIcon, Trash2Icon } from '@lucide/vue'
import { useMailStore } from '@/stores/mail'
import type { MailSearchField, MailSearchFilters } from '@/types/mail'

const emit = defineEmits<{ filters: [filters: MailSearchFilters] }>()
const mailStore = useMailStore()
const query = ref('')
const field = ref<MailSearchField>('all')
const unreadOnly = ref(false)
const hasAttachments = ref(false)

function emitFilters() {
  emit('filters', {
    query: query.value,
    field: field.value,
    unreadOnly: unreadOnly.value,
    hasAttachments: hasAttachments.value,
  })
}

function toggleUnread() {
  unreadOnly.value = !unreadOnly.value
  emitFilters()
}

function toggleAttachments() {
  hasAttachments.value = !hasAttachments.value
  emitFilters()
}

async function onClear() {
  if (confirm('Clear all mails?')) {
    await mailStore.clearAllMails()
  }
}
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  border-bottom: 1px solid var(--border);
}

.search-wrap {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 8px;
  color: var(--text-muted);
  pointer-events: none;
}

.search {
  width: 100%;
  padding: 5px 8px 5px 26px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg-base);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
  transition: border-color 0.15s;
}
.search:focus { border-color: var(--accent); }

.field-select {
  height: 28px;
  max-width: 94px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg-base);
  color: var(--text-secondary);
  font-size: 11px;
  outline: none;
  flex-shrink: 0;
}
.field-select:focus { border-color: var(--accent); }

.filter-btn {
  height: 28px;
  padding: 0 8px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 11px;
  font-weight: 600;
  flex-shrink: 0;
  transition: all 0.15s;
}
.filter-btn.active,
.filter-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--bg-selected);
}

.clear-btn {
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
.clear-btn:hover {
  background: var(--danger);
  border-color: var(--danger);
  color: #fff;
}
</style>
