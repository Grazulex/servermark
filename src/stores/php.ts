import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { PhpVersion } from '@/types'

export const usePhpStore = defineStore('php', () => {
  const versions = ref<PhpVersion[]>([
    { version: '8.3', installed: false, active: false, path: '' },
    { version: '8.2', installed: false, active: false, path: '' },
    { version: '8.1', installed: false, active: false, path: '' },
    { version: '8.0', installed: false, active: false, path: '' },
    { version: '7.4', installed: false, active: false, path: '' },
  ])

  const installedVersions = computed(() => versions.value.filter((v) => v.installed))

  const activeVersion = computed(() => versions.value.find((v) => v.active))

  async function installVersion(version: string) {
    // TODO: Call Tauri command
    const php = versions.value.find((v) => v.version === version)
    if (php) {
      php.installed = true
      php.path = `/usr/bin/php${version}`
    }
  }

  async function uninstallVersion(version: string) {
    // TODO: Call Tauri command
    const php = versions.value.find((v) => v.version === version)
    if (php) {
      php.installed = false
      php.active = false
      php.path = ''
    }
  }

  async function switchVersion(version: string) {
    // TODO: Call Tauri command
    versions.value.forEach((v) => {
      v.active = v.version === version && v.installed
    })
  }

  return {
    versions,
    installedVersions,
    activeVersion,
    installVersion,
    uninstallVersion,
    switchVersion,
  }
})
