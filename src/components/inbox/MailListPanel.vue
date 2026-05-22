<template>
  <div class="list-panel">
    <MailListToolbar @search="q => (query = q)" />
    <MailList v-if="mails.length > 0" :query="query" />
    <EmptyState
      v-else
      title="No mails yet"
      description="Send an email to localhost:1025"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useMailStore } from '@/stores/mail'
import MailListToolbar from './MailListToolbar.vue'
import MailList from './MailList.vue'
import EmptyState from '@/components/ui/EmptyState.vue'

const mailStore = useMailStore()
const { mails } = storeToRefs(mailStore)
const query = ref('')

onMounted(() => mailStore.fetchMails())
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
