import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getConfig, saveConfig, getSmtpStatus } from '@/lib/tauri'
import type { AppConfig } from '@/types/mail'

export const useConfigStore = defineStore('config', () => {
  const config = ref<AppConfig>({
    smtp_port: 1025,
    theme: 'system',
    max_mails: 500,
    check_updates_on_startup: true,
    auto_install_updates: false,
    enable_notifications: true,
  })
  const smtpRunning = ref(false)
  const smtpError = ref<string | null>(null)

  async function loadConfig() {
    config.value = await getConfig()
    applyTheme(config.value.theme)
    setTimeout(async () => {
      smtpRunning.value = await getSmtpStatus()
    }, 800)
  }

  async function updateConfig(updates: Partial<AppConfig>) {
    const next = { ...config.value, ...updates }
    await saveConfig(next)
    config.value = next
    applyTheme(next.theme)
  }

  function applyTheme(theme: AppConfig['theme']) {
    const root = document.documentElement
    if (theme === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.setAttribute('data-theme', prefersDark ? 'dark' : 'light')
    } else {
      root.setAttribute('data-theme', theme)
    }
  }

  return { config, smtpRunning, smtpError, loadConfig, updateConfig, applyTheme }
})
