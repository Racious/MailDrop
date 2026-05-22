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
          :mail="filtered[vRow.index]"
          :is-selected="filtered[vRow.index]?.id === selectedMailId"
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

const props = defineProps<{ query: string }>()

const mailStore = useMailStore()
const { mails, selectedMailId } = storeToRefs(mailStore)

const containerRef = ref<HTMLElement | null>(null)

const filtered = computed(() =>
  props.query
    ? mails.value.filter(
        (m) =>
          m.subject.toLowerCase().includes(props.query.toLowerCase()) ||
          m.from_addr.toLowerCase().includes(props.query.toLowerCase()) ||
          (m.from_name ?? '').toLowerCase().includes(props.query.toLowerCase()),
      )
    : mails.value,
)

const rowVirtualizer = useVirtualizer(
  computed(() => ({
    count: filtered.value.length,
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
