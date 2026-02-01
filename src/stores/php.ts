import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

export interface PhpVersion {
  version: string
  full_version: string
  installed: boolean
  active: boolean
  path: string
}

export interface PhpExtension {
  name: string
  display_name: string
  description: string
  category: 'required' | 'recommended' | 'optional'
  default_selected: boolean
}

export interface PpaStatus {
  installed: boolean
  name: string
  add_command: string
}

export interface InstallProgress {
  step: string
  current_step: number
  total_steps: number
  status: 'running' | 'complete' | 'error'
}

export const usePhpStore = defineStore('php', () => {
  const versions = ref<PhpVersion[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const ppaStatus = ref<PpaStatus | null>(null)
  const extensions = ref<PhpExtension[]>([])
  const addingPpa = ref(false)
  const installing = ref(false)
  const uninstalling = ref(false)
  const ppaProgress = ref<InstallProgress | null>(null)
  const installProgress = ref<InstallProgress | null>(null)
  const uninstallProgress = ref<InstallProgress | null>(null)

  let ppaUnlisten: UnlistenFn | null = null
  let installUnlisten: UnlistenFn | null = null
  let uninstallUnlisten: UnlistenFn | null = null

  const installedVersions = computed(() => versions.value.filter((v) => v.installed))
  const activeVersion = computed(() => versions.value.find((v) => v.active))
  const isPpaInstalled = computed(() => ppaStatus.value?.installed ?? false)

  // Setup event listeners
  async function setupListeners() {
    console.log('[PHP Store] Setting up event listeners...')
    if (!ppaUnlisten) {
      ppaUnlisten = await listen<InstallProgress>('ppa-progress', (event) => {
        console.log('[PHP Store] PPA progress event:', event.payload)
        ppaProgress.value = event.payload
      })
    }
    if (!installUnlisten) {
      installUnlisten = await listen<InstallProgress>('php-install-progress', (event) => {
        console.log('[PHP Store] Install progress event:', event.payload)
        installProgress.value = event.payload
      })
    }
    if (!uninstallUnlisten) {
      uninstallUnlisten = await listen<InstallProgress>('php-uninstall-progress', (event) => {
        console.log('[PHP Store] Uninstall progress event:', event.payload)
        uninstallProgress.value = event.payload
      })
    }
    console.log('[PHP Store] Event listeners ready')
  }

  function cleanupListeners() {
    if (ppaUnlisten) {
      ppaUnlisten()
      ppaUnlisten = null
    }
    if (installUnlisten) {
      installUnlisten()
      installUnlisten = null
    }
    if (uninstallUnlisten) {
      uninstallUnlisten()
      uninstallUnlisten = null
    }
  }

  // Track if listeners are ready
  let listenersReady: Promise<void> | null = null

  // Initialize listeners
  function initListeners() {
    if (!listenersReady) {
      listenersReady = setupListeners()
    }
    return listenersReady
  }

  // Start initialization
  initListeners()

  async function fetchVersions() {
    loading.value = true
    error.value = null
    try {
      const result = await invoke<PhpVersion[]>('get_php_versions')
      versions.value = result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to fetch PHP versions:', e)
    } finally {
      loading.value = false
    }
  }

  async function switchVersion(version: string) {
    loading.value = true
    error.value = null
    try {
      await invoke('switch_php_version', { version })
      // Refresh versions to update active state
      await fetchVersions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to switch PHP version:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function installVersion(version: string, packageManager: string = 'apt') {
    loading.value = true
    error.value = null
    try {
      await invoke('install_php_version', { version, packageManager })
      // Refresh versions after install
      await fetchVersions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to install PHP version:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function checkPpa() {
    try {
      const result = await invoke<PpaStatus>('check_php_ppa')
      ppaStatus.value = result
    } catch (e) {
      console.error('Failed to check PPA status:', e)
    }
  }

  async function addPpa() {
    await initListeners() // Ensure listeners are ready
    addingPpa.value = true
    error.value = null
    ppaProgress.value = null
    try {
      await invoke<string>('add_php_ppa')
      // Re-check PPA status
      await checkPpa()
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to add PPA:', e)
      throw e
    } finally {
      addingPpa.value = false
      // Keep progress visible for a moment after completion
      setTimeout(() => {
        ppaProgress.value = null
      }, 2000)
    }
  }

  async function fetchExtensions() {
    try {
      const result = await invoke<PhpExtension[]>('get_php_extensions')
      extensions.value = result
    } catch (e) {
      console.error('Failed to fetch extensions:', e)
    }
  }

  async function installWithExtensions(
    version: string,
    selectedExtensions: string[],
    packageManager: string = 'apt'
  ) {
    await initListeners() // Ensure listeners are ready
    installing.value = true
    error.value = null
    installProgress.value = null
    const startTime = Date.now()
    try {
      await invoke<string>('install_php_with_extensions', {
        version,
        extensions: selectedExtensions,
        packageManager,
      })
      // Refresh versions after install
      await fetchVersions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to install PHP version:', e)
      throw e
    } finally {
      // Ensure minimum display time of 1.5s for progress
      const elapsed = Date.now() - startTime
      const minDisplayTime = 1500
      if (elapsed < minDisplayTime) {
        await new Promise(resolve => setTimeout(resolve, minDisplayTime - elapsed))
      }
      installing.value = false
      setTimeout(() => {
        installProgress.value = null
      }, 500)
    }
  }

  async function uninstallVersion(version: string, packageManager: string = 'apt') {
    await initListeners() // Ensure listeners are ready
    uninstalling.value = true
    error.value = null
    uninstallProgress.value = null
    const startTime = Date.now()
    try {
      await invoke<string>('uninstall_php_version', {
        version,
        packageManager,
      })
      // Refresh versions after uninstall
      await fetchVersions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to uninstall PHP version:', e)
      throw e
    } finally {
      // Ensure minimum display time of 1.5s for progress
      const elapsed = Date.now() - startTime
      const minDisplayTime = 1500
      if (elapsed < minDisplayTime) {
        await new Promise(resolve => setTimeout(resolve, minDisplayTime - elapsed))
      }
      uninstalling.value = false
      setTimeout(() => {
        uninstallProgress.value = null
      }, 500)
    }
  }

  return {
    versions,
    loading,
    error,
    ppaStatus,
    extensions,
    addingPpa,
    installing,
    uninstalling,
    ppaProgress,
    installProgress,
    uninstallProgress,
    installedVersions,
    activeVersion,
    isPpaInstalled,
    fetchVersions,
    switchVersion,
    installVersion,
    checkPpa,
    addPpa,
    fetchExtensions,
    installWithExtensions,
    uninstallVersion,
    cleanupListeners,
  }
})
