<script setup lang="ts">
import { onMounted } from 'vue'
import { usePhpStore } from '@/stores/php'
import { useConfigStore } from '@/stores/config'

const phpStore = usePhpStore()
const configStore = useConfigStore()

onMounted(async () => {
  await phpStore.fetchVersions()
})

async function handleSwitch(version: string) {
  try {
    await phpStore.switchVersion(version)
  } catch (e) {
    console.error('Failed to switch:', e)
  }
}

async function handleInstall(version: string) {
  try {
    await phpStore.installVersion(version, configStore.config.packageManager)
  } catch (e) {
    console.error('Failed to install:', e)
  }
}
</script>

<template>
  <div class="php-page">
    <header class="page-header">
      <div>
        <h1>PHP Versions</h1>
        <p class="subtitle">
          <template v-if="phpStore.activeVersion">
            Active: PHP {{ phpStore.activeVersion.full_version || phpStore.activeVersion.version }}
          </template>
          <template v-else>
            No PHP detected
          </template>
        </p>
      </div>
      <button
        v-if="phpStore.loading"
        class="btn btn-secondary"
        disabled
      >
        Loading...
      </button>
      <button
        v-else
        class="btn btn-secondary"
        @click="phpStore.fetchVersions()"
      >
        Refresh
      </button>
    </header>

    <div
      v-if="phpStore.error"
      class="error-banner"
    >
      {{ phpStore.error }}
    </div>

    <div class="php-grid">
      <div
        v-for="php in phpStore.versions"
        :key="php.version"
        class="php-card"
        :class="{ active: php.active, installed: php.installed }"
      >
        <div class="php-header">
          <div class="php-version">PHP {{ php.version }}</div>
          <div
            v-if="php.active"
            class="php-badge active"
          >
            Active
          </div>
          <div
            v-else-if="php.installed"
            class="php-badge installed"
          >
            Installed
          </div>
        </div>

        <div
          v-if="php.installed && php.full_version"
          class="php-full-version"
        >
          {{ php.full_version }}
        </div>

        <div
          v-if="php.installed && php.path"
          class="php-path"
        >
          {{ php.path }}
        </div>

        <div class="php-actions">
          <template v-if="!php.installed">
            <button
              class="btn btn-primary"
              :disabled="phpStore.loading"
              @click="handleInstall(php.version)"
            >
              {{ phpStore.loading ? 'Installing...' : 'Install' }}
            </button>
          </template>
          <template v-else>
            <button
              v-if="!php.active"
              class="btn btn-success"
              :disabled="phpStore.loading"
              @click="handleSwitch(php.version)"
            >
              {{ phpStore.loading ? 'Switching...' : 'Set Active' }}
            </button>
            <span
              v-else
              class="active-label"
            >
              Currently Active
            </span>
          </template>
        </div>
      </div>
    </div>

    <div
      v-if="phpStore.versions.length === 0 && !phpStore.loading"
      class="empty-state"
    >
      <div class="empty-icon">P</div>
      <h3>No PHP versions found</h3>
      <p>Click "Refresh" to scan for installed PHP versions.</p>
    </div>
  </div>
</template>

<style scoped>
.php-page {
  max-width: 1200px;
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

.subtitle {
  color: var(--color-text-muted);
  margin-top: 4px;
  font-family: var(--font-mono);
  font-size: 13px;
}

.error-banner {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid var(--color-danger);
  color: var(--color-danger);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 24px;
  font-size: 14px;
}

.php-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.php-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
}

.php-card.active {
  border-color: var(--color-success);
  background: linear-gradient(135deg, var(--color-bg-secondary), rgba(34, 197, 94, 0.05));
}

.php-card.installed:not(.active) {
  border-color: var(--color-primary);
}

.php-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.php-version {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}

.php-badge {
  font-size: 11px;
  font-weight: 600;
  padding: 4px 10px;
  border-radius: 100px;
  text-transform: uppercase;
}

.php-badge.active {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.php-badge.installed {
  background: rgba(59, 130, 246, 0.1);
  color: var(--color-primary);
}

.php-full-version {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  margin-bottom: 8px;
}

.php-path {
  font-size: 12px;
  color: var(--color-text-muted);
  font-family: var(--font-mono);
  margin-bottom: 16px;
  padding: 8px 12px;
  background: var(--color-bg-tertiary);
  border-radius: 6px;
  word-break: break-all;
}

.php-actions {
  display: flex;
  gap: 8px;
}

.btn {
  flex: 1;
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-success {
  background: var(--color-success);
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: var(--color-success-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-bg-hover);
}

.active-label {
  flex: 1;
  text-align: center;
  color: var(--color-success);
  font-size: 13px;
  font-weight: 500;
  padding: 10px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  background: var(--color-bg-secondary);
  border: 1px dashed var(--color-border);
  border-radius: 12px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 16px;
  background: var(--color-bg-tertiary);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-muted);
}

.empty-state h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 8px;
}

.empty-state p {
  color: var(--color-text-muted);
  margin: 0;
}
</style>
