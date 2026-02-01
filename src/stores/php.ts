import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface PhpVersion {
  version: string
  full_version: string
  installed: boolean
  active: boolean
  path: string
}

export const usePhpStore = defineStore('php', () => {
  const versions = ref<PhpVersion[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const installedVersions = computed(() => versions.value.filter((v) => v.installed))
  const activeVersion = computed(() => versions.value.find((v) => v.active))

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

  return {
    versions,
    loading,
    error,
    installedVersions,
    activeVersion,
    fetchVersions,
    switchVersion,
    installVersion,
  }
})
