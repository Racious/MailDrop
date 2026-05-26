/**
 * 本地簽署包版腳本
 * 讀取 src-tauri/updater-keys/maildrop.key 並帶入環境變數後執行 tauri build
 * 使用方式：npm run pack:signed
 */

import { readFileSync } from 'fs'
import { spawnSync } from 'child_process'
import { fileURLToPath } from 'url'
import { dirname, resolve } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const keyPath = resolve(__dirname, '../src-tauri/updater-keys/maildrop.key')

let privateKey
try {
  privateKey = readFileSync(keyPath, 'utf8').trim()
} catch {
  console.error('[pack:signed] 找不到私鑰：', keyPath)
  console.error('請確認 src-tauri/updater-keys/maildrop.key 存在')
  process.exit(1)
}

console.log('[pack:signed] 私鑰已載入，開始 tauri build...\n')

const result = spawnSync(
  'npm',
  ['run', 'tauri', '--', 'build'],
  {
    stdio: 'inherit',
    shell: true,
    env: {
      ...process.env,
      TAURI_SIGNING_PRIVATE_KEY: privateKey,
      TAURI_SIGNING_PRIVATE_KEY_PASSWORD: '',
    },
  }
)

process.exit(result.status ?? 1)
