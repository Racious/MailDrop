# MailDrop

MailDrop 是一款以 Tauri 2 + Vue 3 開發的桌面 Email 測試工具，定位接近 MailHog / Mailpit。它會在本機啟動 SMTP server，接收開發環境送出的 email，並提供桌面 UI 檢視信件內容、原始 MIME、HTML/Text 版本與基本收件紀錄。

目前專案以 Windows 桌面使用情境為主，適合後端或整合測試時取代真實 SMTP 服務，避免測試信件外送到正式信箱。

## 功能

- 本機 SMTP server，預設監聽 `127.0.0.1:1025`
- 收件清單與信件詳情檢視
- HTML / Text / Raw MIME 分頁預覽
- 搜尋、刪除單封信、清空全部信件
- SQLite 本機保存收件資料
- 可設定 SMTP port、主題模式與最大保存信件數
- 收到新信時透過 Tauri event 即時更新 UI
- System tray 與 close-to-tray 行為
- Single instance，重複啟動時聚焦既有視窗

## 技術棧

| Layer | Tech |
| --- | --- |
| Desktop shell | Tauri 2 |
| Frontend | Vue 3, TypeScript, Vite |
| State | Pinia |
| UI | Tailwind CSS v4, scoped CSS, Lucide icons |
| Backend | Rust, Tokio |
| SMTP / MIME | Custom SMTP session handling, `mail-parser` |
| Database | SQLite via `rusqlite` + `r2d2_sqlite` |

## 開發環境需求

- Node.js / npm
- Rust toolchain
- Tauri 2 系統依賴
- Windows 開發時需安裝 WebView2 Runtime

## Quick Start

1. 啟動 MailDrop：

```bash
npm install
npm run start
```

2. 確認 SMTP server 使用預設設定：

```text
host: 127.0.0.1
port: 1025
secure: false
auth: none
```

3. 在要測試的應用程式中，將 SMTP 設定指向 MailDrop。寄出測試信後，信件會出現在 MailDrop inbox。

### Spring Boot 設定範例

若專案使用 `spring-boot-starter-mail`，開發環境可在 `application-dev.yml` 設定：

```yaml
spring:
  mail:
    host: 127.0.0.1
    port: 1025
    username:
    password:
    properties:
      mail:
        smtp:
          auth: false
          starttls:
            enable: false
```

使用 `.properties` 格式則可寫成：

```properties
spring.mail.host=127.0.0.1
spring.mail.port=1025
spring.mail.username=
spring.mail.password=
spring.mail.properties.mail.smtp.auth=false
spring.mail.properties.mail.smtp.starttls.enable=false
```

簡單寄信範例：

```java
import org.springframework.mail.SimpleMailMessage;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.stereotype.Service;

@Service
public class MailTestService {
    private final JavaMailSender mailSender;

    public MailTestService(JavaMailSender mailSender) {
        this.mailSender = mailSender;
    }

    public void sendTestMail() {
        SimpleMailMessage message = new SimpleMailMessage();
        message.setFrom("dev@example.com");
        message.setTo("test@example.com");
        message.setSubject("MailDrop test");
        message.setText("Hello from Spring Boot");
        mailSender.send(message);
    }
}
```

HTML email 可使用 `MimeMessageHelper`：

```java
import jakarta.mail.MessagingException;
import jakarta.mail.internet.MimeMessage;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.mail.javamail.MimeMessageHelper;
import org.springframework.stereotype.Service;

@Service
public class HtmlMailTestService {
    private final JavaMailSender mailSender;

    public HtmlMailTestService(JavaMailSender mailSender) {
        this.mailSender = mailSender;
    }

    public void sendHtmlMail() throws MessagingException {
        MimeMessage message = mailSender.createMimeMessage();
        MimeMessageHelper helper = new MimeMessageHelper(message, "UTF-8");

        helper.setFrom("dev@example.com");
        helper.setTo("test@example.com");
        helper.setSubject("MailDrop HTML test");
        helper.setText("<strong>Hello from Spring Boot</strong>", true);

        mailSender.send(message);
    }
}
```

若 Spring Boot 版本仍使用 `javax.mail`，請將 import 改為 `javax.mail.*` 對應套件。

## 安裝與啟動

```bash
npm install
npm run start
```

`npm run start` 會執行 `tauri dev`，並依照 `src-tauri/tauri.conf.json` 先啟動 Vite dev server。

若只需要啟動前端開發伺服器：

```bash
npm run dev
```

建立前端 production bundle：

```bash
npm run build
```

打包桌面應用：

```bash
npm run pack
```

## SMTP 測試方式

啟動 MailDrop 後，讓測試程式將 SMTP host 指向：

```text
host: 127.0.0.1
port: 1025
```

例如使用 Nodemailer：

```ts
import nodemailer from 'nodemailer'

const transporter = nodemailer.createTransport({
  host: '127.0.0.1',
  port: 1025,
  secure: false,
})

await transporter.sendMail({
  from: 'dev@example.com',
  to: 'test@example.com',
  subject: 'MailDrop test',
  text: 'Hello from MailDrop',
  html: '<strong>Hello from MailDrop</strong>',
})
```

或使用 `swaks`：

```bash
swaks --server 127.0.0.1:1025 --from dev@example.com --to test@example.com --data "Subject: MailDrop test\n\nHello from MailDrop"
```

## 專案結構

```text
MailDrop/
├─ src/                         # Vue frontend
│  ├─ components/
│  │  ├─ inbox/                 # Mail list, toolbar, list item
│  │  ├─ layout/                # App shell and sidebar
│  │  ├─ preview/               # HTML/Text/Raw preview
│  │  └─ ui/                    # Shared UI components
│  ├─ composables/              # Tauri event and theme hooks
│  ├─ lib/tauri.ts              # Frontend invoke wrappers
│  ├─ stores/                   # Pinia stores
│  ├─ types/                    # TypeScript models
│  └─ views/SettingsView.vue    # Settings page
├─ src-tauri/                   # Rust backend and Tauri config
│  ├─ src/
│  │  ├─ commands/              # Tauri commands
│  │  ├─ db/                    # SQLite pool, migrations, repository
│  │  ├─ models/                # Rust models serialized to frontend
│  │  ├─ smtp/                  # SMTP server, session parser, MIME handling
│  │  ├─ tray/                  # System tray setup
│  │  └─ lib.rs                 # Tauri setup and AppState
│  └─ tauri.conf.json
├─ docs/                        # Planning / design notes
├─ public/
└─ package.json
```

## 資料保存

MailDrop 會在 Tauri app data directory 建立 SQLite database：

```text
<app-data-dir>/maildrop.db
```

啟動時會自動建立下列表：

- `mails`：保存收件內容、寄件者、收件者、主旨、HTML/Text body、Raw MIME、大小與接收時間
- `app_config`：保存 `smtp_port`、`theme`、`max_mails`

預設設定：

| Key | Default |
| --- | --- |
| `smtp_port` | `1025` |
| `theme` | `system` |
| `max_mails` | `500` |

## Frontend 與 Backend 溝通

Frontend 透過 `@tauri-apps/api/core` 的 `invoke()` 呼叫 Rust commands。主要命令如下：

| Command | Purpose |
| --- | --- |
| `list_mails` | 分頁取得信件摘要 |
| `get_mail` | 取得單封信件詳情 |
| `delete_mail` | 刪除單封信件 |
| `clear_mails` | 清空全部信件 |
| `get_mail_count` | 取得信件總數 |
| `get_config` | 讀取設定 |
| `save_config` | 保存設定 |
| `get_smtp_status` | 取得 SMTP server 狀態 |
| `restart_app` | 重新啟動應用程式 |

收到新信時，Rust backend 會 emit `mail:received` event，前端由 `useMailEvents()` 監聽並更新 Pinia store。

## SMTP 支援範圍

目前 SMTP session 支援開發測試所需的基本命令：

- `EHLO` / `HELO`
- `MAIL FROM`
- `RCPT TO`
- `DATA`
- `RSET`
- `NOOP`
- `QUIT`
- `AUTH`
- `STARTTLS`

注意：`AUTH` 目前直接視為成功，`STARTTLS` 會回覆 ready，但尚未實作真正 TLS upgrade。這符合本機開發工具的 MVP 取向，但不應視為正式 SMTP server 安全實作。

## 已知注意事項

- SMTP port 變更後需要重新啟動應用程式才會生效。
- App 關閉視窗時預設隱藏到 system tray；請從 tray menu 或應用流程結束程式。
- HTML 預覽以 iframe 呈現，請維持開發測試用途，不建議載入不可信的外部內容。
- `docs/vue-concurrent-scroll.md` 是早期企畫文件，目前存在編碼顯示問題；實作狀態請以程式碼與本 README 為準。
