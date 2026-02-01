<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { usePhpStore } from '@/stores/php'
import { useConfigStore } from '@/stores/config'
import PhpInstallModal from '@/components/PhpInstallModal.vue'

const phpStore = usePhpStore()
const configStore = useConfigStore()

const showInstallModal = ref(false)
const selectedVersionToInstall = ref('')

onMounted(async () => {
  await Promise.all([
    phpStore.fetchVersions(),
    phpStore.checkPpa(),
  ])
})

async function handleSwitch(version: string) {
  try {
    await phpStore.switchVersion(version)
  } catch (e) {
    console.error('Failed to switch:', e)
  }
}

function openInstallModal(version: string) {
  selectedVersionToInstall.value = version
  showInstallModal.value = true
}

function closeInstallModal() {
  showInstallModal.value = false
  selectedVersionToInstall.value = ''
}

async function handleUninstall(version: string) {
  if (!confirm(`Are you sure you want to uninstall PHP ${version}? This will remove all related packages.`)) {
    return
  }
  try {
    await phpStore.uninstallVersion(version, configStore.config.packageManager)
  } catch (e) {
    console.error('Failed to uninstall:', e)
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

    <!-- PPA Warning Banner -->
    <div
      v-if="phpStore.ppaStatus && !phpStore.ppaStatus.installed && configStore.config.packageManager === 'apt'"
      class="ppa-banner"
    >
      <div class="ppa-icon">!</div>
      <div class="ppa-content">
        <strong>PPA ondrej/php not detected</strong>
        <p v-if="!phpStore.addingPpa">
          For the latest PHP versions on Ubuntu/Debian, add the ondrej/php PPA.
        </p>
        <!-- PPA Progress -->
        <div
          v-if="phpStore.addingPpa"
          class="ppa-loading"
        >
          <div class="ppa-spinner"></div>
          <span>Adding PPA... Please wait</span>
        </div>
      </div>
      <button
        v-if="!phpStore.addingPpa"
        class="btn btn-warning"
        @click="phpStore.addPpa()"
      >
        Add PPA
      </button>
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
              :disabled="phpStore.loading || phpStore.installing"
              @click="openInstallModal(php.version)"
            >
              {{ phpStore.installing ? 'Installing...' : 'Install' }}
            </button>
          </template>
          <template v-else>
            <div class="actions-row">
              <button
                v-if="!php.active"
                class="btn btn-success"
                :disabled="phpStore.loading || phpStore.uninstalling"
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
            </div>
            <button
              v-if="!php.active"
              class="btn btn-danger btn-sm"
              :disabled="phpStore.uninstalling"
              @click="handleUninstall(php.version)"
            >
              {{ phpStore.uninstalling ? 'Removing...' : 'Uninstall' }}
            </button>
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

    <!-- Install Modal -->
    <PhpInstallModal
      :version="selectedVersionToInstall"
      :show="showInstallModal"
      @close="closeInstallModal"
      @installed="phpStore.fetchVersions()"
    />

    <!-- Loading Overlay for install/uninstall -->
    <div
      v-if="phpStore.installing || phpStore.uninstalling"
      class="loading-overlay"
    >
      <div class="loading-modal">
        <div class="loading-spinner"></div>
        <div class="loading-text">
          {{ phpStore.installing ? 'Installing PHP...' : 'Uninstalling PHP...' }}
        </div>
        <div class="loading-subtext">
          This may take a few minutes. Please wait...
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.php-page {
  max-width: 1200px;
  position: relative;
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

.ppa-banner {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.3);
  border-radius: 12px;
  margin-bottom: 24px;
}

.ppa-icon {
  width: 36px;
  height: 36px;
  background: rgba(245, 158, 11, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 18px;
  color: #f59e0b;
  flex-shrink: 0;
}

.ppa-content {
  flex: 1;
}

.ppa-content strong {
  color: #f59e0b;
  display: block;
  font-size: 14px;
  margin-bottom: 2px;
}

.ppa-content p {
  color: var(--color-text-secondary);
  font-size: 13px;
  margin: 0;
}

.btn-warning {
  background: #f59e0b;
  color: #000;
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.btn-warning:hover:not(:disabled) {
  background: #d97706;
}

.btn-warning:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.ppa-loading {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.ppa-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid rgba(245, 158, 11, 0.3);
  border-top-color: #f59e0b;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
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
  flex-direction: column;
  gap: 8px;
}

.actions-row {
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

.btn-danger {
  background: var(--color-danger);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #dc2626;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
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

/* Loading Overlay */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  backdrop-filter: blur(4px);
}

.loading-modal {
  background: var(--color-bg-primary);
  border-radius: 16px;
  padding: 48px 64px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  border: 1px solid var(--color-border);
}

.loading-spinner {
  width: 56px;
  height: 56px;
  border: 4px solid var(--color-bg-tertiary);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.loading-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.loading-subtext {
  font-size: 14px;
  color: var(--color-text-muted);
}
</style>
