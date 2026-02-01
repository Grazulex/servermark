<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePhpStore, useSitesStore } from '@/stores'
import { useDockerStore } from '@/stores/docker'
import { useSystemStore } from '@/stores/system'

interface WebServerStatus {
  caddy_installed: boolean
  caddy_running: boolean
  caddy_version: string | null
  nginx_installed: boolean
  nginx_running: boolean
  nginx_version: string | null
  active: string | null
}

const phpStore = usePhpStore()
const sitesStore = useSitesStore()
const dockerStore = useDockerStore()
const systemStore = useSystemStore()

const webServer = ref<WebServerStatus | null>(null)
const webServerLoading = ref(false)
const webServerInstalling = ref(false)
const webServerSwitching = ref(false)

const hasWebServer = computed(() => {
  return webServer.value?.caddy_installed || webServer.value?.nginx_installed
})

const activeWebServer = computed(() => {
  return webServer.value?.active || null
})

async function detectWebServer() {
  webServerLoading.value = true
  try {
    webServer.value = await invoke<WebServerStatus>('detect_web_server')
  } catch (e) {
    console.error('Failed to detect web server:', e)
  } finally {
    webServerLoading.value = false
  }
}

async function installWebServer(server: 'caddy' | 'nginx') {
  if (!systemStore.info?.package_manager) {
    console.error('Package manager not detected')
    return
  }
  webServerInstalling.value = true
  try {
    await invoke('install_web_server', {
      server,
      packageManager: systemStore.info.package_manager
    })
    await detectWebServer()
  } catch (e) {
    console.error('Failed to install web server:', e)
  } finally {
    webServerInstalling.value = false
  }
}

async function switchWebServer(server: 'caddy' | 'nginx') {
  webServerSwitching.value = true
  try {
    await invoke('switch_web_server', { server })
    await detectWebServer()
  } catch (e) {
    console.error('Failed to switch web server:', e)
  } finally {
    webServerSwitching.value = false
  }
}

onMounted(async () => {
  // Detect system info first (needed for package manager)
  if (!systemStore.info) {
    await systemStore.detectSystem()
  }
  // Load PHP versions if not already loaded
  if (phpStore.versions.length === 0) {
    await phpStore.fetchVersions()
  }
  // Load sites
  await sitesStore.fetchSites()
  // Load Docker info
  await dockerStore.detectRuntime()
  if (dockerStore.isAvailable) {
    await dockerStore.listContainers()
  }
  // Detect web server
  await detectWebServer()
})
</script>

<template>
  <div class="dashboard">
    <header class="page-header">
      <h1>Dashboard</h1>
      <p class="subtitle">ServerMark - Local Development Environment</p>
    </header>

    <!-- Quick Stats -->
    <section class="stats-grid">
      <div class="stat-card">
        <div class="stat-value">{{ dockerStore.runningContainers.length }}</div>
        <div class="stat-label">Containers Running</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ sitesStore.sites.length }}</div>
        <div class="stat-label">Sites Configured</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ phpStore.activeVersion?.version || '-' }}</div>
        <div class="stat-label">Active PHP</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ phpStore.installedVersions.length }}</div>
        <div class="stat-label">PHP Versions</div>
      </div>
    </section>

    <!-- Web Server Status -->
    <section class="section">
      <div class="section-header">
        <h2>Web Server</h2>
      </div>

      <div v-if="webServerLoading" class="webserver-card loading">
        <span>Detecting web server...</span>
      </div>

      <div v-else-if="!hasWebServer" class="webserver-card not-installed">
        <div class="webserver-warning">
          <span class="warning-icon">!</span>
          <span>No web server installed</span>
        </div>
        <div class="install-buttons">
          <button
            class="install-btn caddy"
            :disabled="webServerInstalling"
            @click="installWebServer('caddy')"
          >
            <span v-if="webServerInstalling">Installing...</span>
            <span v-else>Install Caddy (Recommended)</span>
          </button>
          <button
            class="install-btn nginx"
            :disabled="webServerInstalling"
            @click="installWebServer('nginx')"
          >
            <span v-if="webServerInstalling">Installing...</span>
            <span v-else>Install Nginx</span>
          </button>
        </div>
      </div>

      <div v-else class="webserver-card installed">
        <div class="webserver-switch">
          <button
            class="switch-btn"
            :class="{ active: activeWebServer === 'caddy', disabled: !webServer?.caddy_installed }"
            :disabled="webServerSwitching || !webServer?.caddy_installed || activeWebServer === 'caddy'"
            @click="switchWebServer('caddy')"
          >
            <span class="switch-status" :class="{ running: webServer?.caddy_running }"></span>
            <span class="switch-label">Caddy</span>
            <span v-if="webServer?.caddy_version" class="switch-version">{{ webServer.caddy_version }}</span>
            <span v-if="!webServer?.caddy_installed" class="switch-not-installed">Not installed</span>
          </button>

          <button
            class="switch-btn"
            :class="{ active: activeWebServer === 'nginx', disabled: !webServer?.nginx_installed }"
            :disabled="webServerSwitching || !webServer?.nginx_installed || activeWebServer === 'nginx'"
            @click="switchWebServer('nginx')"
          >
            <span class="switch-status" :class="{ running: webServer?.nginx_running }"></span>
            <span class="switch-label">Nginx</span>
            <span v-if="webServer?.nginx_version" class="switch-version">{{ webServer.nginx_version }}</span>
            <span v-if="!webServer?.nginx_installed" class="switch-not-installed">Not installed</span>
          </button>
        </div>

        <div v-if="!webServer?.caddy_installed || !webServer?.nginx_installed" class="install-missing">
          <button
            v-if="!webServer?.caddy_installed"
            class="install-btn-small"
            :disabled="webServerInstalling"
            @click="installWebServer('caddy')"
          >
            + Install Caddy
          </button>
          <button
            v-if="!webServer?.nginx_installed"
            class="install-btn-small"
            :disabled="webServerInstalling"
            @click="installWebServer('nginx')"
          >
            + Install Nginx
          </button>
        </div>
      </div>
    </section>

    <!-- Containers Overview -->
    <section class="section">
      <div class="section-header">
        <h2>Containers</h2>
        <router-link
          to="/containers"
          class="link"
        >
          View all
        </router-link>
      </div>
      <div v-if="!dockerStore.isAvailable" class="no-docker">
        <span class="warning-icon">!</span>
        <span>Docker not detected</span>
      </div>
      <div v-else-if="dockerStore.containers.length === 0" class="empty-containers">
        No containers running
      </div>
      <div v-else class="containers-list">
        <div
          v-for="container in dockerStore.containers.slice(0, 4)"
          :key="container.id"
          class="container-item"
          :class="{ running: container.status === 'running' }"
        >
          <div class="container-icon">{{ container.name.replace('servermark-', '').charAt(0).toUpperCase() }}</div>
          <div class="container-info">
            <div class="container-name">{{ container.name.replace('servermark-', '') }}</div>
            <div class="container-image">{{ container.image }}</div>
          </div>
          <div class="container-status" :class="container.status">
            <span class="status-dot"></span>
            {{ container.status }}
          </div>
        </div>
      </div>
    </section>

    <!-- Quick Actions -->
    <section class="section">
      <h2>Quick Actions</h2>
      <div class="actions-grid">
        <button class="action-btn">
          <span class="action-icon">+</span>
          <span class="action-label">Add Site</span>
        </button>
        <button class="action-btn">
          <span class="action-icon">P</span>
          <span class="action-label">Install PHP</span>
        </button>
        <button class="action-btn">
          <span class="action-icon">C</span>
          <span class="action-label">Open Config</span>
        </button>
        <button class="action-btn">
          <span class="action-icon">T</span>
          <span class="action-label">Open Terminal</span>
        </button>
      </div>
    </section>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: 1200px;
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

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
  text-align: center;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: var(--color-primary);
  font-family: var(--font-mono);
}

.stat-label {
  font-size: 13px;
  color: var(--color-text-muted);
  margin-top: 4px;
}

.section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section h2 {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.link {
  font-size: 14px;
  color: var(--color-primary);
  text-decoration: none;
}

.link:hover {
  text-decoration: underline;
}

.services-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.action-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  border-color: var(--color-primary);
  background: var(--color-primary-light);
}

.action-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-bg-tertiary);
  border-radius: 10px;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.action-btn:hover .action-icon {
  background: var(--color-primary);
  color: white;
}

.action-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
}

/* Containers */
.no-docker,
.empty-containers {
  padding: 20px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  text-align: center;
  color: var(--color-text-muted);
  font-size: 14px;
}

.no-docker {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border-color: var(--color-warning);
}

.warning-icon {
  width: 24px;
  height: 24px;
  background: rgba(245, 158, 11, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  color: var(--color-warning);
}

.containers-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.container-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 10px;
}

.container-item.running {
  border-left: 3px solid var(--color-success);
}

.container-icon {
  width: 36px;
  height: 36px;
  background: var(--color-bg-tertiary);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-primary);
}

.container-info {
  flex: 1;
}

.container-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-transform: capitalize;
}

.container-image {
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: var(--font-mono);
}

.container-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 500;
  text-transform: capitalize;
}

.container-status .status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-text-muted);
}

.container-status.running {
  color: var(--color-success);
}

.container-status.running .status-dot {
  background: var(--color-success);
}

.container-status.stopped {
  color: var(--color-text-muted);
}

/* Web Server */
.webserver-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
}

.webserver-card.loading {
  text-align: center;
  color: var(--color-text-muted);
}

.webserver-card.not-installed {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.webserver-warning {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--color-warning);
}

.install-buttons {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.install-btn {
  padding: 12px 24px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid var(--color-border);
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.install-btn:hover:not(:disabled) {
  border-color: var(--color-primary);
  background: var(--color-primary-light);
}

.install-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.install-btn.caddy {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.install-btn.caddy:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.webserver-switch {
  display: flex;
  gap: 12px;
}

.switch-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: var(--color-bg-tertiary);
  border: 2px solid var(--color-border);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.switch-btn:hover:not(:disabled) {
  border-color: var(--color-primary);
}

.switch-btn.active {
  border-color: var(--color-success);
  background: rgba(34, 197, 94, 0.1);
}

.switch-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.switch-btn:disabled {
  cursor: not-allowed;
}

.switch-status {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--color-text-muted);
}

.switch-status.running {
  background: var(--color-success);
  box-shadow: 0 0 8px var(--color-success);
}

.switch-label {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.switch-version {
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: var(--font-mono);
}

.switch-not-installed {
  font-size: 11px;
  color: var(--color-text-muted);
  font-style: italic;
}

.install-missing {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--color-border);
}

.install-btn-small {
  padding: 6px 12px;
  font-size: 12px;
  background: transparent;
  border: 1px dashed var(--color-border);
  border-radius: 6px;
  color: var(--color-text-muted);
  cursor: pointer;
  transition: all 0.15s ease;
}

.install-btn-small:hover:not(:disabled) {
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.install-btn-small:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
