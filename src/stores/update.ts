import { defineStore } from 'pinia'
import { ref, shallowRef } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { openUrl } from '@tauri-apps/plugin-opener'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

const RELEASES_URL = 'https://github.com/Racious/MailDrop/releases/latest'

export const useUpdateStore = defineStore('update', () => {
  const currentVersion = ref('')
  const latestVersion = ref<string | null>(null)
  const releaseNotes = ref<string | null>(null)
  const releaseDate = ref<string | null>(null)
  const checking = ref(false)
  const installing = ref(false)
  const updateAvailable = ref(false)
  const lastCheckedAt = ref<string | null>(null)
  const statusMessage = ref<string | null>(null)
  const errorMessage = ref<string | null>(null)
  const pendingUpdate = shallowRef<Update | null>(null)
  const downloadProgress = ref<number | null>(null)

  async function loadCurrentVersion() {
    currentVersion.value = await getVersion()
  }

  async function checkForUpdates(options: { autoInstall?: boolean; silent?: boolean } = {}) {
    if (checking.value || installing.value) return

    checking.value = true
    errorMessage.value = null
    statusMessage.value = options.silent ? null : 'Checking for updates...'

    try {
      if (!currentVersion.value) await loadCurrentVersion()

      const update = await check()
      lastCheckedAt.value = new Date().toISOString()
      pendingUpdate.value = update
      updateAvailable.value = !!update
      latestVersion.value = update?.version ?? null
      releaseNotes.value = update?.body ?? null
      releaseDate.value = update?.date ?? null

      if (!update) {
        statusMessage.value = options.silent ? null : 'You are using the latest version.'
        return
      }

      statusMessage.value = `Version ${update.version} is available.`
      if (options.autoInstall) await installUpdate()
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : String(error)
      if (!options.silent) statusMessage.value = null
    } finally {
      checking.value = false
    }
  }

  async function installUpdate() {
    if (!pendingUpdate.value || installing.value) return

    installing.value = true
    downloadProgress.value = null
    errorMessage.value = null
    statusMessage.value = 'Downloading update...'

    try {
      let contentLength = 0
      let downloaded = 0

      await pendingUpdate.value.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength ?? 0
            downloadProgress.value = 0
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            downloadProgress.value = contentLength > 0
              ? Math.round((downloaded / contentLength) * 100)
              : null
            break
          case 'Finished':
            downloadProgress.value = 100
            break
        }
      })

      statusMessage.value = 'Update installed. Restarting...'
      await relaunch()
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : String(error)
      downloadProgress.value = null
    } finally {
      installing.value = false
    }
  }

  async function openReleasePage() {
    await openUrl(RELEASES_URL)
  }

  return {
    currentVersion,
    latestVersion,
    releaseNotes,
    releaseDate,
    checking,
    installing,
    updateAvailable,
    lastCheckedAt,
    statusMessage,
    errorMessage,
    downloadProgress,
    loadCurrentVersion,
    checkForUpdates,
    installUpdate,
    openReleasePage,
  }
})
