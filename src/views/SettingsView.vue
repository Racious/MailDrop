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

    <div class="actions">
      <button class="btn btn-primary" :disabled="saving" @click="onSave">
        {{ saving ? 'Saving…' : 'Save Settings' }}
      </button>
    </div>
  </div>

  <!-- Restart dialog -->
  <Transition name="overlay">
    <div v-if="showRestartDialog" class="overlay">
      <div class="dialog">
        <h3 class="dialog-title">需要重新啟動</h3>
        <p class="dialog-msg">SMTP Port 已變更，需要重新啟動應用程式才能生效。</p>
        <div class="dialog-actions">
          <button class="btn btn-ghost" @click="showRestartDialog = false">稍後再說</button>
          <button class="btn btn-primary" @click="onRestart">立即重啟</button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useConfigStore } from '@/stores/config'
import { restartApp } from '@/lib/tauri'
import type { AppConfig } from '@/types/mail'

const configStore = useConfigStore()
const { config } = storeToRefs(configStore)

const draft = reactive<AppConfig>({ ...config.value })
const saving = ref(false)
const showRestartDialog = ref(false)

watch(config, (c) => Object.assign(draft, c), { deep: true })

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
</script>

<style scoped>
.settings {
  padding: 24px 32px;
  max-width: 480px;
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
  width: 200px;
  transition: border-color 0.15s;
}
.input:focus { border-color: var(--accent); }

.hint {
  font-size: 11px;
  color: var(--text-muted);
  margin: 0;
}

.actions { margin-top: 32px; }

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
.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
}
.btn-ghost:hover { background: var(--bg-hover); }

/* ── Restart dialog ─────────────────────────────────────────────────── */
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
