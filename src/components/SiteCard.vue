<script setup lang="ts">
import { computed } from 'vue'
import type { Site } from '@/types'

const props = defineProps<{
  site: Site
}>()

const emit = defineEmits<{
  open: []
  terminal: []
  settings: []
  secure: []
  unsecure: []
  upgradeLaravel: []
  fixPermissions: []
  toggleScheduler: []
  toggleQueue: []
  viewLogs: []
  remove: []
}>()

const typeIcon = computed(() => {
  const icons: Record<string, string> = {
    laravel: 'L',
    symfony: 'S',
    wordpress: 'W',
    static: 'H',
    proxy: 'P',
  }
  return icons[props.site.site_type] || '?'
})

const typeColor = computed(() => {
  const colors: Record<string, string> = {
    laravel: '#ff2d20',
    symfony: '#000000',
    wordpress: '#21759b',
    static: '#4a5568',
    proxy: '#805ad5',
  }
  return colors[props.site.site_type] || '#4a5568'
})

const hasLaravelUpdate = computed(() => {
  return props.site.laravel?.detected && props.site.laravel?.has_update
})
</script>

<template>
  <div class="site-card" :class="{ secured: site.secured }">
    <div class="site-header">
      <div
        class="site-icon"
        :style="{ backgroundColor: typeColor }"
      >
        {{ typeIcon }}
      </div>
      <div class="site-info">
        <div class="site-name">{{ site.name }}</div>
        <a
          :href="`http${site.secured ? 's' : ''}://${site.domain}`"
          target="_blank"
          class="site-domain"
        >
          {{ site.domain }}
        </a>
      </div>
      <div class="site-badges">
        <span
          v-if="site.secured"
          class="badge badge-success"
        >
          SSL
        </span>
        <span
          v-else
          class="badge badge-muted"
        >
          HTTP
        </span>
      </div>
    </div>

    <div class="site-meta">
      <div class="meta-item">
        <span class="meta-label">PHP</span>
        <span class="meta-value">{{ site.php_version }}</span>
      </div>
      <div class="meta-item">
        <span class="meta-label">Type</span>
        <span class="meta-value capitalize">{{ site.site_type }}</span>
      </div>
      <div
        v-if="site.laravel?.detected"
        class="meta-item"
      >
        <span class="meta-label">Laravel</span>
        <span class="meta-value">
          {{ site.laravel.version || site.laravel.constraint }}
          <span
            v-if="hasLaravelUpdate"
            class="update-badge"
            :title="`Update available: ${site.laravel?.latest_version}`"
          >
            !
          </span>
        </span>
      </div>
    </div>

    <div class="site-path">
      <span class="path-label">Path:</span>
      <span class="path-value">{{ site.path }}</span>
    </div>

    <div class="site-actions">
      <!-- Toggle buttons (activation switches) -->
      <div class="actions-toggles">
        <button
          class="btn btn-icon"
          :class="site.secured ? 'btn-success' : 'btn-secondary'"
          @click="site.secured ? emit('unsecure') : emit('secure')"
          :title="site.secured ? 'Disable HTTPS' : 'Enable HTTPS'"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          </svg>
        </button>
        <button
          v-if="site.site_type === 'laravel'"
          class="btn btn-icon"
          :class="site.laravel?.scheduler_enabled ? 'btn-success' : 'btn-secondary'"
          @click="emit('toggleScheduler')"
          :title="site.laravel?.scheduler_enabled ? 'Disable scheduler' : 'Enable scheduler'"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
        </button>
        <button
          v-if="site.site_type === 'laravel'"
          class="btn btn-icon"
          :class="site.laravel?.queue_enabled ? 'btn-success' : 'btn-secondary'"
          @click="emit('toggleQueue')"
          :title="site.laravel?.queue_enabled ? 'Stop queue worker' : 'Start queue worker'"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="8" y1="6" x2="21" y2="6"/>
            <line x1="8" y1="12" x2="21" y2="12"/>
            <line x1="8" y1="18" x2="21" y2="18"/>
            <line x1="3" y1="6" x2="3.01" y2="6"/>
            <line x1="3" y1="12" x2="3.01" y2="12"/>
            <line x1="3" y1="18" x2="3.01" y2="18"/>
          </svg>
        </button>
      </div>
      <!-- Action buttons -->
      <div class="actions-buttons">
        <button
          class="btn btn-icon btn-primary"
          @click="emit('open')"
          title="Open in browser"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
            <polyline points="15 3 21 3 21 9"/>
            <line x1="10" y1="14" x2="21" y2="3"/>
          </svg>
        </button>
        <button
          class="btn btn-icon btn-secondary"
          @click="emit('terminal')"
          title="Open terminal"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="4 17 10 11 4 5"/>
            <line x1="12" y1="19" x2="20" y2="19"/>
          </svg>
        </button>
        <button
          v-if="site.site_type === 'laravel'"
          class="btn btn-icon btn-secondary"
          @click="emit('viewLogs')"
          title="View logs"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10 9 9 9 8 9"/>
          </svg>
        </button>
        <button
          v-if="site.site_type === 'laravel'"
          class="btn btn-icon btn-secondary"
          @click="emit('fixPermissions')"
          title="Fix Laravel permissions"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
          </svg>
        </button>
        <button
          v-if="hasLaravelUpdate"
          class="btn btn-icon btn-warning"
          @click="emit('upgradeLaravel')"
          title="Upgrade Laravel"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="19" x2="12" y2="5"/>
            <polyline points="5 12 12 5 19 12"/>
          </svg>
        </button>
        <button
          class="btn btn-icon btn-secondary"
          @click="emit('settings')"
          title="Site settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>
        <button
          class="btn btn-icon btn-danger-outline"
          @click="emit('remove')"
          title="Remove site"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.site-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
  transition: all 0.2s ease;
}

.site-card:hover {
  border-color: var(--color-border-hover);
}

.site-card.secured {
  border-left: 3px solid var(--color-success);
}

.site-card:not(.secured) {
  border-left: 3px solid var(--color-text-muted);
}

.site-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.site-icon {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  color: white;
  flex-shrink: 0;
}

.site-info {
  flex: 1;
  min-width: 0;
}

.site-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.site-domain {
  font-size: 13px;
  color: var(--color-primary);
  font-family: var(--font-mono);
  text-decoration: none;
}

.site-domain:hover {
  text-decoration: underline;
}

.site-badges {
  display: flex;
  gap: 6px;
}

.badge {
  font-size: 10px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 4px;
  text-transform: uppercase;
}

.badge-success {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.badge-muted {
  background: var(--color-bg-tertiary);
  color: var(--color-text-muted);
}

.site-meta {
  display: flex;
  gap: 20px;
  margin-bottom: 12px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.meta-label {
  font-size: 11px;
  color: var(--color-text-muted);
  text-transform: uppercase;
}

.meta-value {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
  font-family: var(--font-mono);
  display: flex;
  align-items: center;
  gap: 6px;
}

.capitalize {
  text-transform: capitalize;
}

.update-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: var(--color-warning);
  color: white;
  border-radius: 50%;
  font-size: 10px;
  font-weight: 700;
  cursor: help;
}

.site-path {
  font-size: 12px;
  padding: 8px 12px;
  background: var(--color-bg-tertiary);
  border-radius: 6px;
  margin-bottom: 16px;
  font-family: var(--font-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.path-label {
  color: var(--color-text-muted);
  margin-right: 8px;
}

.path-value {
  color: var(--color-text-secondary);
}

.site-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.actions-toggles,
.actions-buttons {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 8px 14px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-icon {
  width: 36px;
  height: 36px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon svg {
  flex-shrink: 0;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-success {
  background: var(--color-success);
  color: white;
}

.btn-success:hover {
  background: var(--color-success-hover);
}

.btn-warning {
  background: var(--color-warning);
  color: white;
}

.btn-warning:hover {
  background: var(--color-warning-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover {
  background: var(--color-bg-hover);
}

.btn-danger-outline {
  background: transparent;
  border: 1px solid var(--color-danger);
  color: var(--color-danger);
}

.btn-danger-outline:hover {
  background: var(--color-danger);
  color: white;
}
</style>
