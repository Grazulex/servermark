<script setup lang="ts">
import { useConfigStore } from '@/stores'

const configStore = useConfigStore()
</script>

<template>
  <div class="settings-page">
    <header class="page-header">
      <h1>Settings</h1>
      <p class="subtitle">Configure your ServerMark environment</p>
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
              v-model="configStore.config.tld"
              type="text"
              placeholder="test"
            />
            <p class="setting-help">Sites will be accessible at *.{{ configStore.config.tld }}</p>
          </div>

          <div class="setting-item">
            <label for="loopback">Loopback Address</label>
            <input
              id="loopback"
              v-model="configStore.config.loopback"
              type="text"
              placeholder="127.0.0.1"
            />
            <p class="setting-help">IP address for local sites</p>
          </div>

          <div class="setting-item">
            <label for="sitesPath">Sites Directory</label>
            <div class="input-with-button">
              <input
                id="sitesPath"
                v-model="configStore.config.sitesPath"
                type="text"
                placeholder="/home/user/Sites"
              />
              <button class="btn btn-secondary">
                Browse
              </button>
            </div>
            <p class="setting-help">Directory where your projects are located</p>
          </div>
        </div>
      </section>

      <!-- Web Server Settings -->
      <section class="settings-section">
        <h2>Web Server</h2>
        <div class="radio-group">
          <label class="radio-option">
            <input
              v-model="configStore.config.webServer"
              type="radio"
              value="nginx"
            />
            <span class="radio-label">
              <strong>Nginx</strong>
              <span>Traditional, widely used web server</span>
            </span>
          </label>
          <label class="radio-option">
            <input
              v-model="configStore.config.webServer"
              type="radio"
              value="caddy"
            />
            <span class="radio-label">
              <strong>Caddy</strong>
              <span>Modern, automatic HTTPS, simpler config</span>
            </span>
          </label>
        </div>
      </section>

      <!-- System Info -->
      <section class="settings-section">
        <h2>System Information</h2>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Distribution</span>
            <span class="info-value">{{ configStore.config.distro || 'Detecting...' }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Package Manager</span>
            <span class="info-value">{{ configStore.config.packageManager }}</span>
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
  margin-bottom: 32px;
}

.page-header h1 {
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0;
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

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.radio-option {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.radio-option:hover {
  border-color: var(--color-primary);
}

.radio-option input {
  margin-top: 2px;
}

.radio-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.radio-label strong {
  color: var(--color-text-primary);
}

.radio-label span {
  font-size: 13px;
  color: var(--color-text-muted);
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
