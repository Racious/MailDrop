import { onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useMailStore } from '@/stores/mail'
import type { MailSummary } from '@/types/mail'

export function useMailEvents() {
  const mailStore = useMailStore()
  let unlisten: (() => void) | null = null

  onMounted(async () => {
    unlisten = await listen<MailSummary>('mail:received', (event) => {
      mailStore.prependMail(event.payload)
    })
  })

  onUnmounted(() => {
    unlisten?.()
  })
}
