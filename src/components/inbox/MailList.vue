<template>
  <div ref="containerRef" class="mail-list">
    <div :style="{ height: `${totalSize}px`, position: 'relative' }">
      <div
        v-for="vRow in virtualRows"
        :key="String(vRow.key)"
        :style="{
          position: 'absolute',
          top: 0,
          left: 0,
          width: '100%',
          transform: `translateY(${vRow.start}px)`,
        }"
      >
        <MailListItem
          :mail="mails[vRow.index]"
          :is-selected="mails[vRow.index]?.id === selectedMailId"
          @select="mailStore.fetchMailDetail"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { storeToRefs } from 'pinia'
import { useMailStore } from '@/stores/mail'
import MailListItem from './MailListItem.vue'

const mailStore = useMailStore()
const { mails, selectedMailId } = storeToRefs(mailStore)

const containerRef = ref<HTMLElement | null>(null)

const rowVirtualizer = useVirtualizer(
  computed(() => ({
    count: mails.value.length,
    getScrollElement: () => containerRef.value,
    estimateSize: () => 72,
    overscan: 5,
  })),
)

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems())
const totalSize = computed(() => rowVirtualizer.value.getTotalSize())
</script>

<style scoped>
.mail-list {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}
</style>
