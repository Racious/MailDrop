export interface MailSummary {
  id: string
  from_addr: string
  from_name: string | null
  subject: string
  received_at: string
  size_bytes: number
  has_html: boolean
  is_read: boolean
}

export interface Mail extends MailSummary {
  message_id: string | null
  to_addrs: string[]
  text_body: string | null
  html_body: string | null
  raw_mime: string
}

export interface AppConfig {
  smtp_port: number
  theme: 'light' | 'dark' | 'system'
  max_mails: number
  check_updates_on_startup: boolean
  auto_install_updates: boolean
  enable_notifications: boolean
}
