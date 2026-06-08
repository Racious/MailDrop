<template>
  <div class="settings">
    <h2 class="settings-title">Settings</h2>

    <section class="section">
      <h3 class="section-title">SMTP Server</h3>
      <div class="field">
        <label class="label" for="smtp-port">Listen Port</label>
        <input
          id="smtp-port"
          v-model.number="draft.smtp_port"
          class="input"
          type="number"
          min="1"
          max="65535"
        />
        <p class="hint">Restart required for port changes to take effect.</p>
      </div>
    </section>

    <section class="section">
      <h3 class="section-title">Appearance</h3>
      <div class="field">
        <label class="label" for="theme">Theme</label>
        <select id="theme" v-model="draft.theme" class="input">
          <option value="light">Light</option>
          <option value="dark">Dark</option>
          <option value="system">System</option>
        </select>
      </div>
    </section>

    <section class="section">
      <h3 class="section-title">Storage</h3>
      <div class="field">
        <label class="label" for="max-mails">Max mails stored</label>
        <input
          id="max-mails"
          v-model.number="draft.max_mails"
          class="input"
          type="number"
          min="10"
          max="10000"
        />
      </div>
    </section>

    <section class="section">
      <h3 class="section-title">Notifications</h3>
      <label class="check-field">
        <input v-model="draft.enable_notifications" type="checkbox" />
        <span>Show desktop notification when new mail arrives</span>
      </label>
    </section>

    <section class="section">
      <h3 class="section-title">Updates</h3>
      <div class="version-row">
        <span>Current version</span>
        <strong>{{ currentVersion || 'Unknown' }}</strong>
      </div>
      <label class="check-field">
        <input v-model="draft.check_updates_on_startup" type="checkbox" />
        <span>Check for updates when MailDrop starts</span>
      </label>
      <label class="check-field">
        <input v-model="draft.auto_install_updates" type="checkbox" />
        <span>Automatically download and install available updates</span>
      </label>
      <div class="update-actions">
        <button class="btn btn-ghost" :disabled="checking || installing" @click="onCheckUpdates">
          {{ checking ? 'Checking...' : 'Check for updates' }}
        </button>
        <button
          v-if="updateAvailable"
          class="btn btn-primary"
          :disabled="installing"
          @click="onInstallUpdate"
        >
          {{ installing ? 'Installing...' : `Install ${latestVersion}` }}
        </button>
        <button class="btn btn-ghost" @click="onOpenReleasePage">
          Open releases
        </button>
      </div>
      <div v-if="installing && downloadProgress !== null" class="progress-wrap">
        <div class="progress-bar" :style="{ width: downloadProgress + '%' }" />
        <span class="progress-label">{{ downloadProgress }}%</span>
      </div>
      <p v-if="statusMessage" class="hint">{{ statusMessage }}</p>
      <p v-if="errorMessage" class="hint hint-danger">{{ errorMessage }}</p>
    </section>

    <section class="section">
      <div class="section-heading">
        <h3 class="section-title">SMTP Session Log</h3>
        <button class="btn btn-ghost btn-small" type="button" @click="loadSessions">Refresh</button>
      </div>
      <div v-if="sessions.length" class="session-list">
        <details v-for="session in sessions" :key="session.id" class="session-item">
          <summary>
            <span>{{ session.mail_id ? 'Accepted mail' : 'No mail captured' }}</span>
            <span class="session-time">{{ formatDate(session.started_at) }}</span>
          </summary>
          <p v-if="session.error" class="hint hint-danger">{{ session.error }}</p>
          <pre>{{ session.transcript }}</pre>
        </details>
      </div>
      <p v-else class="hint">No SMTP sessions recorded yet.</p>
      <p class="hint">REST API is available locally at http://127.0.0.1:8025/api/messages</p>
    </section>

    <div class="actions">
      <button class="btn btn-primary" :disabled="saving" @click="onSave">
        {{ saving ? 'Saving...' : 'Save Settings' }}
      </button>
    </div>
  </div>

  <Transition name="overlay">
    <div v-if="showRestartDialog" class="overlay">
      <div class="dialog">
        <h3 class="dialog-title">Restart required</h3>
        <p class="dialog-msg">SMTP port changes take effect after restarting MailDrop.</p>
        <div class="dialog-actions">
          <button class="btn btn-ghost" @click="showRestartDialog = false">Later</button>
          <button class="btn btn-primary" @click="onRestart">Restart now</button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useConfigStore } from '@/stores/config'
import { useUpdateStore } from '@/stores/update'
import { listSmtpSessions, restartApp } from '@/lib/tauri'
import { formatDate } from '@/lib/utils'
import type { AppConfig, SmtpSessionLog } from '@/types/mail'

const configStore = useConfigStore()
const updateStore = useUpdateStore()
const { config } = storeToRefs(configStore)
const {
  currentVersion,
  latestVersion,
  checking,
  installing,
  updateAvailable,
  statusMessage,
  errorMessage,
  downloadProgress,
} = storeToRefs(updateStore)

const draft = reactive<AppConfig>({ ...config.value })
const saving = ref(false)
const showRestartDialog = ref(false)
const sessions = ref<SmtpSessionLog[]>([])

watch(config, (c) => Object.assign(draft, c), { deep: true })
onMounted(() => void loadSessions())

async function onSave() {
  const portChanged = draft.smtp_port !== config.value.smtp_port
  saving.value = true
  try {
    await configStore.updateConfig({ ...draft })
    if (portChanged) showRestartDialog.value = true
  } finally {
    saving.value = false
  }
}

async function onRestart() {
  showRestartDialog.value = false
  await restartApp()
}

async function onCheckUpdates() {
  await updateStore.checkForUpdates({ autoInstall: draft.auto_install_updates })
}

async function onInstallUpdate() {
  await updateStore.installUpdate()
}

async function onOpenReleasePage() {
  await updateStore.openReleasePage()
}

async function loadSessions() {
  sessions.value = await listSmtpSessions(20)
}
</script>

<style scoped>
.settings {
  padding: 24px 32px;
  max-width: 560px;
  overflow-y: auto;
  height: 100%;
}

.settings-title {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 24px;
}

.section {
  margin-bottom: 28px;
}

.section-title {
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin: 0 0 12px;
}

.section-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.btn-small {
  padding: 5px 10px;
  font-size: 11px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.input {
  padding: 7px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg-surface);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  width: 220px;
  transition: border-color 0.15s;
}
.input:focus { border-color: var(--accent); }

.hint {
  font-size: 11px;
  color: var(--text-muted);
  margin: 0;
}
.hint-danger { color: var(--danger); }

.version-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 300px;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}
.version-row strong { color: var(--text-primary); }

.check-field {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 9px 0;
  font-size: 13px;
  color: var(--text-primary);
}
.check-field input {
  width: 15px;
  height: 15px;
}

.update-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 14px 0 8px;
}

.progress-wrap {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 8px 0 4px;
  width: 300px;
}

.progress-bar {
  flex: 1;
  height: 4px;
  background: var(--accent);
  border-radius: 2px;
  transition: width 0.2s ease;
}

.progress-label {
  font-size: 11px;
  color: var(--text-muted);
  min-width: 30px;
  text-align: right;
}

.actions { margin-top: 32px; }

.session-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.session-item {
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
}

.session-item summary {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 8px 10px;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 600;
}

.session-time {
  color: var(--text-muted);
  font-weight: 400;
}

.session-item pre {
  margin: 0;
  padding: 10px;
  max-height: 220px;
  overflow: auto;
  border-top: 1px solid var(--border);
  color: var(--text-secondary);
  font-size: 11px;
  line-height: 1.5;
  white-space: pre-wrap;
}

.btn {
  padding: 8px 20px;
  border-radius: 6px;
  border: none;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s;
}
.btn-primary {
  background: var(--accent);
  color: #fff;
}
.btn-primary:hover:not(:disabled) { background: var(--accent-hover); }
.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
}
.btn-ghost:hover:not(:disabled) { background: var(--bg-hover); }

.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.dialog {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 24px;
  width: 360px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.dialog-title {
  font-size: 15px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 10px;
}

.dialog-msg {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0 0 20px;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.overlay-enter-active, .overlay-leave-active { transition: opacity 0.2s; }
.overlay-enter-from, .overlay-leave-to { opacity: 0; }
</style>
