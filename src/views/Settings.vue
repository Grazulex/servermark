<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

interface SitesConfig {
  tld: string
  sites_path: string
  loopback?: string
}

interface SystemInfo {
  distro: string
  distro_version: string
  package_manager: string
}

const config = ref<SitesConfig>({
  tld: 'test',
  sites_path: '',
  loopback: '127.0.0.1',
})

const systemInfo = ref<SystemInfo>({
  distro: 'Detecting...',
  distro_version: '',
  package_manager: 'apt',
})

const saving = ref(false)
const saved = ref(false)

onMounted(async () => {
  try {
    const [sitesConfig, sysInfo] = await Promise.all([
      invoke<SitesConfig>('get_sites_config'),
      invoke<SystemInfo>('detect_system'),
    ])
    config.value = sitesConfig
    systemInfo.value = sysInfo
  } catch (e) {
    console.error('Failed to load config:', e)
  }
})

async function browseSitesPath() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Sites Directory',
  })
  if (selected && typeof selected === 'string') {
    config.value.sites_path = selected
  }
}

async function saveConfig() {
  saving.value = true
  saved.value = false
  try {
    await invoke('update_sites_config', {
      tld: config.value.tld,
      sitesPath: config.value.sites_path,
    })
    saved.value = true
    setTimeout(() => { saved.value = false }, 2000)
  } catch (e) {
    console.error('Failed to save config:', e)
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="settings-page">
    <header class="page-header">
      <div>
        <h1>Settings</h1>
        <p class="subtitle">Configure your ServerMark environment</p>
      </div>
      <button
        class="btn btn-primary"
        :disabled="saving"
        @click="saveConfig"
      >
        {{ saving ? 'Saving...' : saved ? 'Saved!' : 'Save Settings' }}
      </button>
    </header>

    <div class="settings-sections">
      <!-- General Settings -->
      <section class="settings-section">
        <h2>General</h2>
        <div class="settings-grid">
          <div class="setting-item">
            <label for="tld">Top Level Domain</label>
            <input
              id="tld"
              v-model="config.tld"
              type="text"
              placeholder="test"
            />
            <p class="setting-help">Sites will be accessible at *.{{ config.tld }}</p>
          </div>

          <div class="setting-item">
            <label for="sitesPath">Sites Directory</label>
            <div class="input-with-button">
              <input
                id="sitesPath"
                v-model="config.sites_path"
                type="text"
                placeholder="/home/user/Sites"
              />
              <button class="btn btn-secondary" @click="browseSitesPath">
                Browse
              </button>
            </div>
            <p class="setting-help">Directory where your projects are located</p>
          </div>
        </div>
      </section>

      <!-- System Info -->
      <section class="settings-section">
        <h2>System Information</h2>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Distribution</span>
            <span class="info-value">{{ systemInfo.distro }} {{ systemInfo.distro_version }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Package Manager</span>
            <span class="info-value">{{ systemInfo.package_manager }}</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  max-width: 800px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.page-header h1 {
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.subtitle {
  color: var(--color-text-muted);
  margin-top: 4px;
}

.settings-sections {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.settings-section {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 24px;
}

.settings-section h2 {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 20px;
}

.settings-grid {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-item label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.setting-item input {
  padding: 10px 14px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}

.setting-item input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.setting-help {
  font-size: 12px;
  color: var(--color-text-muted);
  margin: 0;
}

.input-with-button {
  display: flex;
  gap: 8px;
}

.input-with-button input {
  flex: 1;
}

.btn {
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  cursor: pointer;
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover {
  background: var(--color-bg-hover);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 12px;
  color: var(--color-text-muted);
  text-transform: uppercase;
}

.info-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}
</style>
