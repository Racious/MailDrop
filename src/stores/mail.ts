import { defineStore } from 'pinia'
import { ref } from 'vue'
import { clearMails, deleteMail, getMail, getMailCount, listMails } from '@/lib/tauri'
import type { Mail, MailSummary } from '@/types/mail'

export const useMailStore = defineStore('mail', () => {
  const mails = ref<MailSummary[]>([])
  const selectedMailId = ref<string | null>(null)
  const selectedMail = ref<Mail | null>(null)
  const totalCount = ref(0)
  const loadingList = ref(false)
  const loadingDetail = ref(false)

  async function fetchMails(offset = 0, limit = 100) {
    loadingList.value = true
    try {
      const [items, count] = await Promise.all([
        listMails(offset, limit),
        getMailCount(),
      ])
      mails.value = items
      totalCount.value = count
    } finally {
      loadingList.value = false
    }
  }

  async function fetchMailDetail(id: string) {
    if (selectedMailId.value === id && selectedMail.value) return
    loadingDetail.value = true
    selectedMailId.value = id
    try {
      selectedMail.value = await getMail(id)
    } finally {
      loadingDetail.value = false
    }
  }

  async function removeMail(id: string) {
    await deleteMail(id)
    mails.value = mails.value.filter((m) => m.id !== id)
    totalCount.value = Math.max(0, totalCount.value - 1)
    if (selectedMailId.value === id) {
      selectedMailId.value = null
      selectedMail.value = null
    }
  }

  async function clearAllMails() {
    await clearMails()
    mails.value = []
    totalCount.value = 0
    selectedMailId.value = null
    selectedMail.value = null
  }

  function prependMail(summary: MailSummary) {
    mails.value.unshift(summary)
    totalCount.value++
  }

  return {
    mails,
    selectedMailId,
    selectedMail,
    totalCount,
    loadingList,
    loadingDetail,
    fetchMails,
    fetchMailDetail,
    removeMail,
    clearAllMails,
    prependMail,
  }
})
