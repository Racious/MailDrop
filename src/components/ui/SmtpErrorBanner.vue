<template>
  <Transition name="banner">
    <div v-if="smtpError" class="banner">
      <AlertCircleIcon :size="15" class="banner-icon" />
      <span class="banner-msg">{{ smtpError }}</span>
      <button class="banner-btn" @click="goSettings">前往設定</button>
      <button class="banner-close" @click="configStore.smtpError = null">✕</button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import { storeToRefs } from 'pinia'
import { AlertCircleIcon } from '@lucide/vue'
import { useConfigStore } from '@/stores/config'

const configStore = useConfigStore()
const { smtpError } = storeToRefs(configStore)
const showSettings = inject<Ref<boolean>>('showSettings')!

function goSettings() {
  showSettings.value = true
  configStore.smtpError = null
}
</script>

<style scoped>
.banner {
  position: fixed;
  top: 12px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  border-radius: 8px;
  background: #fef2f2;
  border: 1px solid #fca5a5;
  color: #991b1b;
  font-size: 13px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.12);
  max-width: 500px;
  white-space: nowrap;
}

[data-theme="dark"] .banner {
  background: #450a0a;
  border-color: #7f1d1d;
  color: #fca5a5;
}

.banner-icon { flex-shrink: 0; }
.banner-msg  { flex: 1; }

.banner-btn {
  padding: 4px 10px;
  border-radius: 5px;
  border: 1px solid currentColor;
  background: transparent;
  color: inherit;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
}
.banner-btn:hover { opacity: 0.8; }

.banner-close {
  background: transparent;
  border: none;
  color: inherit;
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
  padding: 0 2px;
  opacity: 0.6;
}
.banner-close:hover { opacity: 1; }

.banner-enter-active, .banner-leave-active { transition: all 0.25s ease; }
.banner-enter-from, .banner-leave-to { opacity: 0; transform: translateX(-50%) translateY(-8px); }
</style>
