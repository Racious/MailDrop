import { defineStore } from 'pinia'
import { ref } from 'vue'
import { clearMails, deleteMail, getMail, getMailCount, getUnreadCount, listMails, markAsRead } from '@/lib/tauri'
import type { Mail, MailSummary } from '@/types/mail'

export const useMailStore = defineStore('mail', () => {
  const mails = ref<MailSummary[]>([])
  const selectedMailId = ref<string | null>(null)
  const selectedMail = ref<Mail | null>(null)
  const totalCount = ref(0)
  const unreadCount = ref(0)
  const loadingList = ref(false)
  const loadingDetail = ref(false)

  async function fetchMails(offset = 0, limit = 100) {
    loadingList.value = true
    try {
      const [items, count, unread] = await Promise.all([
        listMails(offset, limit),
        getMailCount(),
        getUnreadCount(),
      ])
      mails.value = items
      totalCount.value = count
      unreadCount.value = unread
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
      const mail = mails.value.find((m) => m.id === id)
      if (mail && !mail.is_read) {
        mail.is_read = true
        unreadCount.value = Math.max(0, unreadCount.value - 1)
        void markAsRead(id)
      }
    } finally {
      loadingDetail.value = false
    }
  }

  async function removeMail(id: string) {
    const mail = mails.value.find((m) => m.id === id)
    await deleteMail(id)
    mails.value = mails.value.filter((m) => m.id !== id)
    totalCount.value = Math.max(0, totalCount.value - 1)
    if (mail && !mail.is_read) unreadCount.value = Math.max(0, unreadCount.value - 1)
    if (selectedMailId.value === id) {
      selectedMailId.value = null
      selectedMail.value = null
    }
  }

  async function clearAllMails() {
    await clearMails()
    mails.value = []
    totalCount.value = 0
    unreadCount.value = 0
    selectedMailId.value = null
    selectedMail.value = null
  }

  function prependMail(summary: MailSummary) {
    mails.value.unshift(summary)
    totalCount.value++
    unreadCount.value++
  }

  return {
    mails,
    selectedMailId,
    selectedMail,
    totalCount,
    unreadCount,
    loadingList,
    loadingDetail,
    fetchMails,
    fetchMailDetail,
    removeMail,
    clearAllMails,
    prependMail,
  }
})
