<template>
  <iframe
    ref="frame"
    class="html-preview"
    sandbox="allow-same-origin allow-popups"
    title="Mail HTML preview"
  />
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'

const props = defineProps<{ html: string }>()
const frame = ref<HTMLIFrameElement | null>(null)

function injectLinkHandler(iframe: HTMLIFrameElement) {
  const doc = iframe.contentDocument
  if (!doc) return
  doc.addEventListener('click', (e) => {
    const anchor = (e.target as HTMLElement).closest('a')
    if (anchor?.href) {
      e.preventDefault()
      openUrl(anchor.href)
    }
  })
}

function setContent(html: string) {
  const iframe = frame.value
  if (!iframe) return
  iframe.srcdoc = html
  iframe.addEventListener('load', () => injectLinkHandler(iframe), { once: true })
}

onMounted(() => setContent(props.html))

watch(() => props.html, (html) => setContent(html))
</script>

<style scoped>
.html-preview {
  flex: 1;
  width: 100%;
  border: none;
  background: #fff;
}
</style>
