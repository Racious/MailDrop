export interface MailSummary {
  id: string
  from_addr: string
  from_name: string | null
  subject: string
  received_at: string
  size_bytes: number
  has_html: boolean
  is_read: boolean
  attachment_count: number
}

export interface Mail extends MailSummary {
  message_id: string | null
  to_addrs: string[]
  text_body: string | null
  html_body: string | null
  raw_mime: string
  attachments: MailAttachment[]
}

export interface MailAttachment {
  id: string
  mail_id: string
  filename: string
  content_type: string
  size_bytes: number
}

export interface SmtpSessionLog {
  id: string
  mail_id: string | null
  remote_addr: string
  started_at: string
  ended_at: string
  transcript: string
  error: string | null
}

export type MailSearchField = 'all' | 'from' | 'to' | 'subject' | 'body' | 'attachments'

export interface MailSearchFilters {
  query: string
  field: MailSearchField
  unreadOnly: boolean
  hasAttachments: boolean
}

export interface MailSearchResult {
  items: MailSummary[]
  total_count: number
  unread_count: number
}

export interface AppConfig {
  smtp_port: number
  theme: 'light' | 'dark' | 'system'
  max_mails: number
  check_updates_on_startup: boolean
  auto_install_updates: boolean
  enable_notifications: boolean
}
