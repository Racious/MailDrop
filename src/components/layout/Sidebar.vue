<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="logo">✉ MailDrop</span>
    </div>

    <nav class="sidebar-nav">
      <button
        class="nav-item"
        :class="{ active: !showSettings }"
        @click="showSettings = false"
      >
        <InboxIcon :size="16" />
        Inbox
        <span v-if="totalCount > 0" class="badge">{{ totalCount }}</span>
      </button>
      <button
        class="nav-item"
        :class="{ active: showSettings }"
        @click="showSettings = true"
      >
        <SettingsIcon :size="16" />
        Settings
      </button>
    </nav>

    <div class="sidebar-footer">
      <SmtpStatusBadge />
      <ThemeToggle />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import { InboxIcon, SettingsIcon } from '@lucide/vue'
import SmtpStatusBadge from '@/components/ui/SmtpStatusBadge.vue'
import ThemeToggle from '@/components/ui/ThemeToggle.vue'
import { useMailStore } from '@/stores/mail'
import { storeToRefs } from 'pinia'

const mailStore = useMailStore()
const { totalCount } = storeToRefs(mailStore)

const showSettings = inject<Ref<boolean>>('showSettings')!
</script>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  background: var(--bg-surface);
  border-right: 1px solid var(--border);
  user-select: none;
}

.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid var(--border);
}

.logo {
  font-weight: 700;
  font-size: 15px;
  color: var(--text-primary);
}

.sidebar-nav {
  flex: 1;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  text-align: left;
  transition: background 0.15s;
}
.nav-item:hover { background: var(--bg-hover); color: var(--text-primary); }
.nav-item.active { background: var(--bg-selected); color: var(--accent); }

.badge {
  margin-left: auto;
  background: var(--accent);
  color: #fff;
  font-size: 11px;
  font-weight: 600;
  padding: 1px 6px;
  border-radius: 10px;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
