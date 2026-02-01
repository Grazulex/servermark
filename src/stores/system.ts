import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface SystemInfo {
  distro: string
  distro_version: string
  package_manager: string
  kernel: string
  hostname: string
}

export const useSystemStore = defineStore('system', () => {
  const info = ref<SystemInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function detectSystem() {
    loading.value = true
    error.value = null
    try {
      info.value = await invoke<SystemInfo>('detect_system')
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  return {
    info,
    loading,
    error,
    detectSystem,
  }
})
