<template>
  <div class="app-shell" :class="{ 'settings-mode': showSettings }">
    <Sidebar />
    <template v-if="!showSettings">
      <MailListPanel />
      <MailPreviewPanel />
    </template>
    <SettingsView v-if="showSettings" />
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import Sidebar from './Sidebar.vue'
import MailListPanel from '@/components/inbox/MailListPanel.vue'
import MailPreviewPanel from '@/components/preview/MailPreviewPanel.vue'
import SettingsView from '@/views/SettingsView.vue'

const showSettings = inject<Ref<boolean>>('showSettings')!
</script>

<style scoped>
.app-shell {
  display: grid;
  grid-template-columns: 240px 320px 1fr;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-base);
}
.app-shell.settings-mode {
  grid-template-columns: 240px 1fr;
}
</style>
