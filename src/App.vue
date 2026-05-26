<script setup lang="ts">
import { onMounted, provide, ref } from 'vue'
import AppShell from '@/components/layout/AppShell.vue'
import SmtpErrorBanner from '@/components/ui/SmtpErrorBanner.vue'
import { useConfigStore } from '@/stores/config'
import { useUpdateStore } from '@/stores/update'
import { useMailEvents } from '@/composables/useMailEvents'
import { useSmtpEvents } from '@/composables/useSmtpEvents'

const showSettings = ref(false)
provide('showSettings', showSettings)

const configStore = useConfigStore()
const updateStore = useUpdateStore()

onMounted(async () => {
  await configStore.loadConfig()
  await updateStore.loadCurrentVersion()
  if (configStore.config.check_updates_on_startup) {
    void updateStore.checkForUpdates({
      autoInstall: configStore.config.auto_install_updates,
      silent: true,
    })
  }
})

useMailEvents()
useSmtpEvents()
</script>

<template>
  <AppShell />
  <SmtpErrorBanner />
</template>
