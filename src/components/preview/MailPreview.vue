<template>
  <div class="mail-preview">
    <PreviewHeader :mail="mail" />
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
import type { Mail } from '@/types/mail'
import PreviewHeader from './PreviewHeader.vue'
import PreviewTabs from './PreviewTabs.vue'
import HtmlPreview from './HtmlPreview.vue'
import TextPreview from './TextPreview.vue'
import RawPreview from './RawPreview.vue'

const props = defineProps<{ mail: Mail }>()

const activeTab = ref<'HTML' | 'Text' | 'Raw'>('HTML')

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
</style>
