<template>
  <div class="html-preview-wrap">
    <div v-if="remoteContentBlocked" class="remote-banner">
      <span>Remote content blocked</span>
      <button type="button" @click="allowRemoteContent = true">Load once</button>
    </div>
    <iframe
      ref="frame"
      class="html-preview"
      sandbox="allow-same-origin allow-popups"
      title="Mail HTML preview"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'

const props = defineProps<{ html: string }>()
const frame = ref<HTMLIFrameElement | null>(null)
const allowRemoteContent = ref(false)
const remoteContentBlocked = ref(false)

function injectLinkHandler(iframe: HTMLIFrameElement) {
  const doc = iframe.contentDocument
  if (!doc) return
  doc.addEventListener('click', (e) => {
    const anchor = (e.target as HTMLElement).closest('a')
    if (!anchor?.href) return
    e.preventDefault()
    const url = new URL(anchor.href)
    if (['http:', 'https:', 'mailto:'].includes(url.protocol)) void openUrl(url.href)
  })
}

function sanitizeHtml(html: string) {
  remoteContentBlocked.value = false
  const doc = new DOMParser().parseFromString(html, 'text/html')
  doc.querySelectorAll('script, object, embed, applet, form').forEach((node) => node.remove())
  doc.querySelectorAll('[src], [srcset], [background]').forEach((node) => {
    for (const attr of ['src', 'srcset', 'background']) {
      const value = node.getAttribute(attr)
      if (value && isRemoteUrl(value) && !allowRemoteContent.value) {
        node.removeAttribute(attr)
        remoteContentBlocked.value = true
      }
    }
  })
  doc.querySelectorAll('a[href]').forEach((node) => {
    const href = node.getAttribute('href') ?? ''
    if (!isAllowedLink(href)) node.removeAttribute('href')
  })
  return `<!doctype html>${doc.documentElement.outerHTML}`
}

function isRemoteUrl(value: string) {
  const trimmed = value.trim().toLowerCase()
  return trimmed.startsWith('http://') || trimmed.startsWith('https://') || trimmed.startsWith('//')
}

function isAllowedLink(value: string) {
  try {
    const url = new URL(value, 'https://maildrop.local')
    return ['http:', 'https:', 'mailto:'].includes(url.protocol)
  } catch {
    return false
  }
}

function setContent(html: string) {
  const iframe = frame.value
  if (!iframe) return
  iframe.srcdoc = sanitizeHtml(html)
  iframe.addEventListener('load', () => injectLinkHandler(iframe), { once: true })
}

onMounted(() => setContent(props.html))

watch(() => props.html, (html) => setContent(html))
watch(allowRemoteContent, () => setContent(props.html))
</script>

<style scoped>
.html-preview-wrap {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.remote-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 7px 12px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-size: 12px;
}

.remote-banner button {
  height: 24px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-base);
  color: var(--text-primary);
  cursor: pointer;
}

.html-preview {
  flex: 1;
  width: 100%;
  border: none;
  background: #fff;
}
</style>
