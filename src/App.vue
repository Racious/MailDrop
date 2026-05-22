<script setup lang="ts">
import { onMounted, provide, ref } from 'vue'
import AppShell from '@/components/layout/AppShell.vue'
import SmtpErrorBanner from '@/components/ui/SmtpErrorBanner.vue'
import { useConfigStore } from '@/stores/config'
import { useMailEvents } from '@/composables/useMailEvents'
import { useSmtpEvents } from '@/composables/useSmtpEvents'

const showSettings = ref(false)
provide('showSettings', showSettings)

const configStore = useConfigStore()
onMounted(() => configStore.loadConfig())

useMailEvents()
useSmtpEvents()
</script>

<template>
  <AppShell />
  <SmtpErrorBanner />
</template>