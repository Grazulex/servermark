import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Config, LinuxDistro } from '@/types'

export const useConfigStore = defineStore('config', () => {
  const config = ref<Config>({
    tld: 'test',
    loopback: '127.0.0.1',
    sitesPath: '',
    webServer: 'nginx',
    distro: 'unknown',
    packageManager: 'apt',
  })

  const distro = ref<LinuxDistro>('unknown')
  const initialized = ref(false)

  async function detectSystem() {
    // TODO: Call Tauri command to detect Linux distribution
    // const info = await invoke('detect_system')
    // distro.value = info.distro
    // config.value.packageManager = info.packageManager
    initialized.value = true
  }

  function updateConfig(updates: Partial<Config>) {
    Object.assign(config.value, updates)
  }

  return {
    config,
    distro,
    initialized,
    detectSystem,
    updateConfig,
  }
})
