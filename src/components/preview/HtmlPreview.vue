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

const props = defineProps<{ html: string }>()
const frame = ref<HTMLIFrameElement | null>(null)

onMounted(() => {
  if (frame.value) frame.value.srcdoc = props.html
})

watch(() => props.html, (html) => {
  if (frame.value) frame.value.srcdoc = html
})
</script>

<style scoped>
.html-preview {
  flex: 1;
  width: 100%;
  border: none;
  background: #fff;
}
</style>
