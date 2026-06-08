import { defineStore } from 'pinia'
import { ref } from 'vue'
import { clearMails, deleteMail, getMail, getMailCount, getUnreadCount, listMails, markAsRead, searchMails } from '@/lib/tauri'
import type { Mail, MailSearchFilters, MailSummary } from '@/types/mail'

export const useMailStore = defineStore('mail', () => {
  const mails = ref<MailSummary[]>([])
  const selectedMailId = ref<string | null>(null)
  const selectedMail = ref<Mail | null>(null)
  const totalCount = ref(0)
  const unreadCount = ref(0)
  const loadingList = ref(false)
  const loadingDetail = ref(false)
  const activeFilters = ref<MailSearchFilters>({
    query: '',
    field: 'all',
    unreadOnly: false,
    hasAttachments: false,
  })

  async function fetchMails(offset = 0, limit = 100, filters = activeFilters.value) {
    loadingList.value = true
    activeFilters.value = { ...filters }
    try {
      const hasAdvancedFilter = Boolean(
        filters.query.trim() || filters.unreadOnly || filters.hasAttachments,
      )
      if (hasAdvancedFilter) {
        const result = await searchMails(filters, offset, limit)
        mails.value = result.items
        totalCount.value = result.total_count
        unreadCount.value = result.unread_count
      } else {
        const [items, count, unread] = await Promise.all([
          listMails(offset, limit),
          getMailCount(),
          getUnreadCount(),
        ])
        mails.value = items
        totalCount.value = count
        unreadCount.value = unread
      }
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
    if (needsRefreshForFilter()) {
      void fetchMails(0, 100)
      return
    }
    if (!matchesActiveFilters(summary)) return
    mails.value.unshift(summary)
    totalCount.value++
    unreadCount.value++
  }

  function needsRefreshForFilter() {
    const filters = activeFilters.value
    return Boolean(filters.query.trim() && ['to', 'body', 'attachments'].includes(filters.field))
  }

  function matchesActiveFilters(mail: MailSummary) {
    const filters = activeFilters.value
    if (filters.unreadOnly && mail.is_read) return false
    if (filters.hasAttachments && mail.attachment_count === 0) return false
    if (!filters.query.trim()) return true
    const query = filters.query.trim().toLowerCase()
    if (filters.field === 'from') {
      return mail.from_addr.toLowerCase().includes(query) || (mail.from_name ?? '').toLowerCase().includes(query)
    }
    if (filters.field === 'subject') return mail.subject.toLowerCase().includes(query)
    return (
      mail.from_addr.toLowerCase().includes(query) ||
      (mail.from_name ?? '').toLowerCase().includes(query) ||
      mail.subject.toLowerCase().includes(query)
    )
  }

  return {
    mails,
    selectedMailId,
    selectedMail,
    totalCount,
    unreadCount,
    loadingList,
    loadingDetail,
    activeFilters,
    fetchMails,
    fetchMailDetail,
    removeMail,
    clearAllMails,
    prependMail,
  }
})
