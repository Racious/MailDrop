<template>
  <div class="list-panel">
    <MailListToolbar @filters="onFiltersChanged" />
    <MailList v-if="mails.length > 0" />
    <EmptyState
      v-else
      title="No mails yet"
      description="Send an email to localhost:1025"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useMailStore } from '@/stores/mail'
import MailListToolbar from './MailListToolbar.vue'
import MailList from './MailList.vue'
import EmptyState from '@/components/ui/EmptyState.vue'
import type { MailSearchFilters } from '@/types/mail'

const mailStore = useMailStore()
const { mails } = storeToRefs(mailStore)
onMounted(() => mailStore.fetchMails())

function onFiltersChanged(next: MailSearchFilters) {
  void mailStore.fetchMails(0, 100, next)
}
</script>

<style scoped>
.list-panel {
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border);
  background: var(--bg-base);
  overflow: hidden;
}
</style>
