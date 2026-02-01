<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useDockerStore } from '@/stores/docker'

const route = useRoute()
const dockerStore = useDockerStore()

const navItems = [
  { path: '/', label: 'Dashboard', icon: 'dashboard' },
  { path: '/sites', label: 'Sites', icon: 'sites' },
  { path: '/containers', label: 'Containers', icon: 'containers' },
  { path: '/databases', label: 'Databases', icon: 'database' },
  { path: '/php', label: 'PHP', icon: 'php' },
  { path: '/settings', label: 'Settings', icon: 'settings' },
]

const isActive = (path: string) => route.path === path

const statusIndicator = computed(() => {
  const running = dockerStore.runningContainers.length
  const stopped = dockerStore.stoppedContainers.length
  const total = dockerStore.containers.length

  if (total === 0) return 'stopped'
  if (running === 0) return 'stopped'
  if (stopped === 0) return 'running'
  return 'partial'
})
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo">
        <span class="logo-icon">S</span>
        <span class="logo-text">ServerMark</span>
      </div>
      <div
        class="status-indicator"
        :class="statusIndicator"
      />
    </div>

    <nav class="sidebar-nav">
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ active: isActive(item.path) }"
      >
        <span class="nav-icon">{{ item.icon.charAt(0).toUpperCase() }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </router-link>
    </nav>

    <div class="sidebar-footer">
      <div class="version">v0.1.0</div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px;
  background: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border);
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  width: 32px;
  height: 32px;
  background: var(--color-primary);
  color: white;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 18px;
}

.logo-text {
  font-weight: 600;
  font-size: 16px;
  color: var(--color-text-primary);
}

.status-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.status-indicator.running {
  background: var(--color-success);
  box-shadow: 0 0 8px var(--color-success);
}

.status-indicator.stopped {
  background: var(--color-text-muted);
}

.status-indicator.partial {
  background: var(--color-warning);
  box-shadow: 0 0 8px var(--color-warning);
}

.sidebar-nav {
  flex: 1;
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-radius: 8px;
  color: var(--color-text-secondary);
  text-decoration: none;
  transition: all 0.15s ease;
}

.nav-item:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.nav-item.active {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.nav-icon {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 12px;
  background: var(--color-bg-tertiary);
  border-radius: 6px;
}

.nav-item.active .nav-icon {
  background: var(--color-primary);
  color: white;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

.sidebar-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--color-border);
}

.version {
  font-size: 12px;
  color: var(--color-text-muted);
}
</style>
