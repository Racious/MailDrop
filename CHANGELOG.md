# Changelog

## v0.1.5

### New Features
- 新增附件解析、保存、附件數標記與附件下載
- 信件預覽新增附件面板，支援 MIME / 大小顯示、圖片縮圖與下載
- 新增後端搜尋，可依寄件者、收件者、主旨、內文與附件檔名查找
- 新增本機 REST API 搜尋參數、附件下載端點與 SMTP session log 查詢
- Settings 新增 SMTP Session Log 檢視區，方便追蹤 SMTP 對話紀錄

### Security / Hardening
- HTML 預覽預設阻擋遠端資源，僅在使用者選擇載入時開放
- HTML 連結僅允許 `http`、`https`、`mailto` 協定
- SMTP session 增加逾時、單行長度與單封信大小限制

### Documentation
- README 補上附件、搜尋、REST API 與 SMTP session log 說明

## v0.1.4

### New Features
- 收到新信時，若視窗未在前景，工具列圖示持續閃爍直到使用者點開視窗
- 應用程式啟動時視窗預設最大化

### Bug Fixes
- 修正 GitHub Release 說明未正確讀取 CHANGELOG 的問題
  - `tauri-action` 不支援 `releaseBodyPath`，改以 step output 傳入 `releaseBody`

## v0.1.3

### Bug Fixes
- 修正自動更新安裝時發生 `Cannot read private member` 錯誤的問題
  - `pendingUpdate` 由 `ref` 改為 `shallowRef`，防止 Vue Proxy 破壞 Tauri `Update` 物件的 private class fields
  - 修正後 v0.1.2 以前的版本無法透過 in-app updater 升級，此版起恢復正常

## v0.1.2

### New Features
- 新增 Windows 系統通知，收到新信時自動顯示 toast（可於 Settings → Notifications 關閉）
- 新增信件未讀狀態：未讀信件顯示藍點 + 加粗主旨，點開後自動標記已讀
- Sidebar Inbox badge 改為顯示未讀數量

## v0.1.1

### Bug Fixes
- 修正 HTML 信件內連結點擊後在 MailDrop 內部顯示白頁的問題，現在會正確開啟系統瀏覽器

## v0.1.0

### Features
- 本機 SMTP server（tokio 非同步，預設 port 1025）
- 收件匣虛擬捲動列表
- HTML / Text / Raw 三種郵件預覽模式
- HTML 預覽信件內連結自動開啟系統瀏覽器
- SQLite 本機保存，可設定最大保存信件數
- 可設定 SMTP port、主題模式（亮色 / 暗色 / 系統跟隨）
- SMTP 監聽失敗時顯示錯誤橫幅並引導至設定頁
- 變更 SMTP port 後提示重啟，支援一鍵自動重啟
- System tray 常駐，右鍵選單支援顯示視窗與結束程式
- Single instance，重複啟動時聚焦既有視窗
- 自動更新功能（Settings 可設定啟動時檢查、自動安裝）
- GitHub Actions 自動 build 並發布 Release
