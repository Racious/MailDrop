<template>
  <div class="preview-panel">
    <MailPreview v-if="selectedMail" :mail="selectedMail" />
    <EmptyState
      v-else
      title="Select a mail to preview"
      description="Click any message in the inbox"
    />
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMailStore } from '@/stores/mail'
import MailPreview from './MailPreview.vue'
import EmptyState from '@/components/ui/EmptyState.vue'

const mailStore = useMailStore()
const { selectedMail } = storeToRefs(mailStore)
</script>

<style scoped>
.preview-panel {
  display: flex;
  flex-direction: column;
  background: var(--bg-base);
  overflow: hidden;
}
</style>
