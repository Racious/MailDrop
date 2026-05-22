import { computed } from 'vue'
import { useConfigStore } from '@/stores/config'

export function useTheme() {
  const configStore = useConfigStore()

  const isDark = computed(() => {
    const theme = configStore.config.theme
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches
    }
    return theme === 'dark'
  })

  function toggle() {
    const next = isDark.value ? 'light' : 'dark'
    configStore.updateConfig({ theme: next })
  }

  return { isDark, toggle }
}
