<template>
  <div class="toolbar">
    <div class="search-wrap">
      <SearchIcon :size="13" class="search-icon" />
      <input
        v-model="query"
        class="search"
        placeholder="Search..."
        type="search"
        @input="$emit('search', query)"
      />
    </div>
    <button class="clear-btn" title="Clear all mails" @click="onClear">
      <Trash2Icon :size="14" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { SearchIcon, Trash2Icon } from '@lucide/vue'
import { useMailStore } from '@/stores/mail'

defineEmits<{ search: [q: string] }>()
const mailStore = useMailStore()
const query = ref('')

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
