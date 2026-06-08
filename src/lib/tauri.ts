import { invoke } from '@tauri-apps/api/core'
import type {
  AppConfig,
  Mail,
  MailAttachment,
  MailSearchFilters,
  MailSearchResult,
  MailSummary,
  SmtpSessionLog,
} from '@/types/mail'

export const listMails = (offset: number, limit: number) =>
  invoke<MailSummary[]>('list_mails', { offset, limit })

export const searchMails = (filters: MailSearchFilters, offset: number, limit: number) =>
  invoke<MailSearchResult>('search_mails', {
    query: filters.query,
    field: filters.field,
    unreadOnly: filters.unreadOnly,
    hasAttachments: filters.hasAttachments,
    offset,
    limit,
  })

export const getMail = (id: string) =>
  invoke<Mail>('get_mail', { id })

export const deleteMail = (id: string) =>
  invoke<string>('delete_mail', { id })

export const clearMails = () =>
  invoke<number>('clear_mails')

export const getMailCount = () =>
  invoke<number>('get_mail_count')

export const markAsRead = (id: string) =>
  invoke<void>('mark_as_read', { id })

export const getUnreadCount = () =>
  invoke<number>('get_unread_count')

export const listAttachments = (mailId: string) =>
  invoke<MailAttachment[]>('list_attachments', { mailId })

export const getAttachmentContent = (mailId: string, attachmentId: string) =>
  invoke<number[]>('get_attachment_content', { mailId, attachmentId })

export const listSmtpSessions = (limit = 100) =>
  invoke<SmtpSessionLog[]>('list_smtp_sessions', { limit })

export const getConfig = () =>
  invoke<AppConfig>('get_config')

export const saveConfig = (config: AppConfig) =>
  invoke<void>('save_config', { config })

export const getSmtpStatus = () =>
  invoke<boolean>('get_smtp_status')

export const restartApp = () =>
  invoke<void>('restart_app')
