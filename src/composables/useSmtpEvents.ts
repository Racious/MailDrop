import { onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useConfigStore } from '@/stores/config'

export function useSmtpEvents() {
  const configStore = useConfigStore()
  const unlisteners: (() => void)[] = []

  onMounted(async () => {
    unlisteners.push(
      await listen<number>('smtp:started', () => {
        configStore.smtpRunning = true
        configStore.smtpError = null
      }),
      await listen<string>('smtp:error', (event) => {
        configStore.smtpRunning = false
        configStore.smtpError = event.payload
      }),
    )
  })

  onUnmounted(() => {
    unlisteners.forEach((fn) => fn())
  })
}
