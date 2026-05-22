# MailDrop — 桌面 Email 測試工具 企畫書

## Context

開發一款類似 MailHog / Mailpit 的開發者 Email 測試桌面工具。  
MailHog 已於 2022 年停止維護（253 open issues），Mailpit 雖為其繼承者但形式為 CLI + 瀏覽器。  
本專案以 **Tauri 2 + Vue 3** 打造 Windows 原生桌面應用，提供系統匣常駐、GUI 設定、深淺色切換等原生體驗，差異化定位為「開發者 GUI 工具」而非 CLI 工具。

**目標平台**：Windows（.exe / .msi）  
**MVP 功能**：SMTP 收信、收件匣列表、HTML/Text/Raw 三模式預覽、深淺色切換、系統匣常駐

---

## 技術棧

| 層 | 技術 |
|---|---|
| 桌面框架 | Tauri 2 |
| 後端 | Rust（tokio, rusqlite, mail-parser） |
| 前端 | Vue 3 + TypeScript + Vite |
| UI | Tailwind CSS v4 + shadcn-vue |
| 狀態管理 | Pinia |
| 儲存 | SQLite（bundled rusqlite） |

---

## 專案目錄結構

```
maildrop/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs                  # Tauri builder、AppState 註冊、SMTP thread 啟動
│       ├── lib.rs                   # AppState 定義
│       ├── commands/
│       │   ├── mail.rs              # list_mails, get_mail, delete_mail, clear_mails, get_mail_count
│       │   └── config.rs            # get_config, save_config
│       ├── smtp/
│       │   ├── server.rs            # TcpListener 接收連線、dispatch
│       │   ├── session.rs           # SMTP 狀態機（最複雜模組）
│       │   └── parser.rs            # mail-parser 封裝
│       ├── db/
│       │   ├── connection.rs        # r2d2 SQLite pool
│       │   ├── migrations.rs        # 內嵌 SQL migration
│       │   └── repository.rs        # CRUD 操作（最核心 DB 介面）
│       ├── tray/
│       │   └── mod.rs               # SystemTray 設定、close-to-tray
│       └── models/
│           ├── mail.rs              # Mail, MailSummary struct
│           └── config.rs            # AppConfig struct
├── src/
│   ├── main.ts
│   ├── App.vue
│   ├── stores/
│   │   ├── mail.ts                  # 收件匣狀態（最核心 store）
│   │   └── config.ts                # 設定與主題狀態
│   ├── composables/
│   │   ├── useMailEvents.ts         # 監聽 mail:received Tauri 事件
│   │   └── useTheme.ts              # 深淺色切換邏輯
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppShell.vue         # 三欄 CSS Grid 佈局
│   │   │   └── Sidebar.vue          # 左欄：狀態、導航
│   │   ├── inbox/
│   │   │   ├── MailList.vue         # 中欄：收件匣列表（虛擬捲動）
│   │   │   ├── MailListItem.vue     # 單筆：寄件人、主旨、時間
│   │   │   └── MailListToolbar.vue  # 搜尋、清空按鈕
│   │   ├── preview/
│   │   │   ├── MailPreview.vue      # 右欄：預覽容器
│   │   │   ├── PreviewTabs.vue      # HTML / Text / Raw 切換
│   │   │   ├── HtmlPreview.vue      # iframe sandbox 渲染（安全關鍵）
│   │   │   ├── TextPreview.vue
│   │   │   └── RawPreview.vue
│   │   └── ui/                      # 共用元件（Button, Badge, ThemeToggle）
│   ├── lib/
│   │   ├── tauri.ts                 # invoke() 型別化封裝
│   │   └── utils.ts                 # 日期格式化、檔案大小工具
│   └── types/
│       └── mail.ts                  # TypeScript interface（對應 Rust model）
└── index.html
```

---

## SQLite Schema

```sql
CREATE TABLE IF NOT EXISTS mails (
    id           TEXT PRIMARY KEY,        -- UUID v4
    message_id   TEXT,
    from_name    TEXT,
    from_addr    TEXT NOT NULL,
    to_addrs     TEXT NOT NULL,           -- JSON array
    subject      TEXT NOT NULL DEFAULT '',
    text_body    TEXT,
    html_body    TEXT,
    raw_mime     TEXT NOT NULL,
    size_bytes   INTEGER NOT NULL DEFAULT 0,
    received_at  TEXT NOT NULL            -- ISO-8601 UTC
);

CREATE INDEX IF NOT EXISTS idx_mails_received_at ON mails(received_at DESC);

CREATE TABLE IF NOT EXISTS app_config (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- 預設值
INSERT OR IGNORE INTO app_config(key, value) VALUES
    ('smtp_port', '1025'),
    ('theme',     'system'),
    ('max_mails', '500');
```

---

## Tauri Commands 介面

```rust
// mail.rs
async fn list_mails(state, offset: u32, limit: u32)  -> Result<Vec<MailSummary>, String>
async fn get_mail(state, id: String)                  -> Result<Mail, String>
async fn delete_mail(state, id: String)               -> Result<String, String>
async fn clear_mails(state)                           -> Result<usize, String>
async fn get_mail_count(state)                        -> Result<u32, String>

// config.rs
async fn get_config(state)                            -> Result<AppConfig, String>
async fn save_config(state, app_handle, config)       -> Result<(), String>
```

後端事件：`app_handle.emit("mail:received", MailSummary)` — 每收到一封信觸發

---

## Rust 模型

```rust
pub struct MailSummary {
    pub id: String,
    pub from_addr: String,
    pub from_name: Option<String>,
    pub subject: String,
    pub received_at: String,
    pub size_bytes: u32,
    pub has_html: bool,
}

pub struct Mail {
    // MailSummary 欄位（flatten）+
    pub to_addrs: Vec<String>,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub raw_mime: String,
}

pub struct AppConfig {
    pub smtp_port: u16,
    pub theme: String,   // "light" | "dark" | "system"
    pub max_mails: u32,
}
```

---

## 前端元件樹

```
App.vue
└── AppShell.vue  (grid-template-columns: 240px 320px 1fr)
    ├── Sidebar.vue
    │   ├── SmtpStatusBadge.vue
    │   └── ThemeToggle.vue
    ├── MailListPanel.vue
    │   ├── MailListToolbar.vue  (搜尋 + 清空)
    │   ├── MailList.vue         (虛擬捲動)
    │   │   └── MailListItem.vue × n
    │   └── EmptyState.vue
    └── MailPreviewPanel.vue
        ├── PreviewHeader.vue    (寄件人、主旨、時間、刪除)
        ├── PreviewTabs.vue
        ├── HtmlPreview.vue      (iframe sandbox，不含 allow-scripts)
        ├── TextPreview.vue
        └── RawPreview.vue

SettingsView.vue  (v-show 切換，非路由)
```

---

## Pinia Store 結構

### `stores/mail.ts`
- **State**：`mails: MailSummary[]`、`selectedMailId`、`selectedMail: Mail | null`、`totalCount`、loading flags
- **Actions**：`fetchMails(offset, limit)`、`fetchMailDetail(id)`、`deleteMail(id)`、`clearAllMails()`、`prependMail(summary)` ← 由事件觸發

### `stores/config.ts`
- **State**：`config: AppConfig`、`smtpRunning: boolean`
- **Actions**：`loadConfig()`、`saveConfig(updates)`、`applyTheme(theme)` ← 操作 `html[data-theme]`

### `composables/useMailEvents.ts`
```typescript
await listen<MailSummary>('mail:received', (event) => {
  mailStore.prependMail(event.payload)
})
```

---

## 關鍵架構決策

| 決策 | 原因 |
|---|---|
| SMTP server 在獨立 OS thread + 獨立 Tokio runtime | 避免長連線阻塞 Tauri 內部 async runtime |
| rusqlite + `spawn_blocking` | rusqlite 為同步，在命令邊界包裝以不阻塞 async handler |
| iframe `srcdoc` 不含 `allow-scripts` | 防止測試郵件中的 JS 執行，安全性邊界 |
| app_config 存 SQLite KV，不用 JSON 檔 | 單一資料檔、避免 Windows 檔案鎖競態 |
| MailSummary vs Mail 分離 | 列表不載入 body，節省記憶體；點選後才 fetch 詳情 |

---

## 開發里程碑

### Phase 1 — MVP（預估 13 工作天）

| # | 工作項目 | 估時 |
|---|---|---|
| 1.1 | 專案腳手架：Tauri init、Vue 3 + TS、Tailwind v4、shadcn-vue | 0.5 天 |
| 1.2 | Rust：AppState、SQLite pool、embedded migrations | 1 天 |
| 1.3 | Rust：SMTP 狀態機（EHLO/MAIL FROM/RCPT TO/DATA/QUIT） | 2 天 |
| 1.4 | Rust：mail-parser 整合，ParsedMail → Mail model | 1 天 |
| 1.5 | Rust：db/repository.rs CRUD | 1 天 |
| 1.6 | Rust：Tauri commands 串接 + emit("mail:received") | 0.5 天 |
| 1.7 | Rust：System tray + close-to-tray | 0.5 天 |
| 1.8 | Frontend：AppShell 三欄佈局、Sidebar、狀態徽章 | 1 天 |
| 1.9 | Frontend：MailList + Pinia mailStore + fetchMails | 1 天 |
| 1.10 | Frontend：useMailEvents 即時更新 | 0.5 天 |
| 1.11 | Frontend：MailPreview HTML/Text/Raw 三模式 | 1 天 |
| 1.12 | Frontend：刪除單筆 / 清空全部 | 0.5 天 |
| 1.13 | Frontend：ThemeToggle + useTheme + CSS 變數切換 | 0.5 天 |
| 1.14 | Frontend：SettingsView port 設定 | 0.5 天 |
| 1.15 | Windows 打包：.msi/.exe、圖示資產 | 0.5 天 |
| 1.16 | 手動 QA：swaks / Nodemailer 發信測試 | 1 天 |

### Phase 2 — 進階功能（預估 16 工作天）

| # | 工作項目 | 估時 |
|---|---|---|
| 2.1 | 附件解析與下載 | 2 天 |
| 2.2 | 全文搜尋（SQLite FTS5） | 1.5 天 |
| 2.3 | 已讀/未讀狀態、系統匣計數徽章 | 1 天 |
| 2.4 | 虛擬捲動（@tanstack/vue-virtual） | 1 天 |
| 2.5 | SMTP AUTH LOGIN/PLAIN 支援 | 1 天 |
| 2.6 | STARTTLS stub（接受命令，不實際 TLS） | 1 天 |
| 2.7 | .eml 匯出（Tauri 檔案對話框） | 0.5 天 |
| 2.8 | 桌面通知（tauri-plugin-notification） | 0.5 天 |
| 2.9 | 開機自動啟動（tauri-plugin-autostart） | 0.5 天 |
| 2.10 | 多 port 設定檔支援 | 2 天 |
| 2.11 | Header 展開檢視器 | 1 天 |
| 2.12 | 鍵盤導航（方向鍵、Delete、Escape） | 1 天 |
| 2.13 | GitHub Actions CI/CD Windows 自動建置 | 1 天 |
| 2.14 | tauri-plugin-updater 自動更新 | 1 天 |

---

## 關鍵 Cargo 依賴

```toml
tauri       = { version = "2", features = ["tray-icon"] }
tokio       = { version = "1", features = ["full"] }
mail-parser = "0.9"
rusqlite    = { version = "0.31", features = ["bundled"] }
r2d2        = "0.8"
r2d2_sqlite = "0.24"
uuid        = { version = "1", features = ["v4"] }
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
chrono      = { version = "0.4", features = ["serde"] }
```

---

## 驗證方式

1. **SMTP 接收**：使用 `swaks --to test@test.com --server localhost:1025` 發送測試郵件
2. **即時更新**：確認 UI 在收信後自動出現新項目（無需手動重新整理）
3. **HTML 渲染**：發送含 `<style>` 與圖片的 HTML 郵件，確認 iframe 正確顯示但 JS 不執行
4. **系統匣**：關閉主視窗後，再發送郵件確認仍可接收；點擊系統匣圖示恢復視窗
5. **主題切換**：切換深淺色確認所有元件 CSS 變數正確應用
6. **打包**：`cargo tauri build` 產出 .msi，安裝後確認可正常執行
