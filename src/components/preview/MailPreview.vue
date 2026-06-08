<template>
  <div class="mail-preview">
    <PreviewHeader :mail="mail" />
    <div v-if="mail.attachments.length" class="attachments-panel">
      <div class="attachments-header">
        <span>Attachments</span>
        <strong>{{ mail.attachments.length }}</strong>
      </div>
      <div class="attachments-grid">
        <div
        v-for="attachment in mail.attachments"
        :key="attachment.id"
        class="attachment-card"
        :title="`${attachment.content_type} · ${formatSize(attachment.size_bytes)}`"
      >
          <img
            v-if="isImageAttachment(attachment.content_type)"
            class="attachment-thumb"
            :src="attachmentUrl(attachment.id)"
            alt=""
          />
          <div v-else class="attachment-icon">
            <ImageIcon v-if="attachment.content_type.startsWith('image/')" :size="18" />
            <FileTextIcon v-else-if="isTextAttachment(attachment.content_type)" :size="18" />
            <FileIcon v-else :size="18" />
          </div>
          <div class="attachment-copy">
            <span class="attachment-name">{{ attachment.filename }}</span>
            <span class="attachment-meta">{{ attachment.content_type }} · {{ formatSize(attachment.size_bytes) }}</span>
          </div>
          <button
            class="attachment-download"
            type="button"
            title="Download attachment"
            @click="downloadAttachment(attachment.id)"
          >
            <DownloadIcon :size="15" />
          </button>
        </div>
      </div>
    </div>
    <PreviewTabs
      v-model="activeTab"
      :has-html="!!mail.html_body"
      :has-text="!!mail.text_body"
    />
    <div class="preview-body">
      <HtmlPreview v-if="activeTab === 'HTML' && mail.html_body" :html="mail.html_body" />
      <TextPreview v-else-if="activeTab === 'Text' && mail.text_body" :text="mail.text_body" />
      <RawPreview v-else-if="activeTab === 'Raw'" :raw="mail.raw_mime" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { DownloadIcon, FileIcon, FileTextIcon, ImageIcon } from '@lucide/vue'
import type { Mail } from '@/types/mail'
import { formatSize } from '@/lib/utils'
import { openUrl } from '@tauri-apps/plugin-opener'
import PreviewHeader from './PreviewHeader.vue'
import PreviewTabs from './PreviewTabs.vue'
import HtmlPreview from './HtmlPreview.vue'
import TextPreview from './TextPreview.vue'
import RawPreview from './RawPreview.vue'

const props = defineProps<{ mail: Mail }>()

const activeTab = ref<'HTML' | 'Text' | 'Raw'>('HTML')

async function downloadAttachment(id: string) {
  await openUrl(attachmentUrl(id))
}

function attachmentUrl(id: string) {
  return `http://127.0.0.1:8025/api/messages/${props.mail.id}/attachments/${id}`
}

function isImageAttachment(contentType: string) {
  return contentType.startsWith('image/')
}

function isTextAttachment(contentType: string) {
  return contentType.startsWith('text/') || contentType.includes('json') || contentType.includes('xml')
}

watch(
  () => props.mail,
  (mail) => {
    if (mail.html_body) activeTab.value = 'HTML'
    else if (mail.text_body) activeTab.value = 'Text'
    else activeTab.value = 'Raw'
  },
  { immediate: true },
)
</script>

<style scoped>
.mail-preview {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.preview-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.attachments-panel {
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-surface);
}

.attachments-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 8px;
  font-size: 11px;
  font-weight: 700;
  color: var(--text-muted);
  text-transform: uppercase;
}

.attachments-header strong {
  color: var(--accent);
}

.attachments-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 8px;
}

.attachment-card {
  display: grid;
  grid-template-columns: 38px minmax(0, 1fr) 30px;
  align-items: center;
  gap: 8px;
  min-height: 46px;
  padding: 6px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-base);
  color: var(--text-secondary);
  font-size: 12px;
}

.attachment-card:hover {
  border-color: var(--accent);
}

.attachment-thumb,
.attachment-icon {
  width: 38px;
  height: 34px;
  border-radius: 5px;
  background: var(--bg-hover);
}

.attachment-thumb {
  object-fit: cover;
}

.attachment-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}

.attachment-copy {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.attachment-name {
  color: var(--text-primary);
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.attachment-meta {
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.attachment-download {
  width: 28px;
  height: 28px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.attachment-download:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--bg-selected);
}
</style>
