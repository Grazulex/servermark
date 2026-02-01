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
  <div class="site-card">
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
      <button
        class="btn btn-primary"
        @click="emit('open')"
      >
        Open
      </button>
      <button
        class="btn btn-secondary"
        @click="emit('terminal')"
        title="Open terminal in project folder"
      >
        Terminal
      </button>
      <button
        v-if="!site.secured"
        class="btn btn-success"
        @click="emit('secure')"
      >
        Secure
      </button>
      <button
        v-else
        class="btn btn-secondary"
        @click="emit('unsecure')"
      >
        Unsecure
      </button>
      <button
        v-if="hasLaravelUpdate"
        class="btn btn-warning"
        @click="emit('upgradeLaravel')"
      >
        Upgrade Laravel
      </button>
      <button
        class="btn btn-secondary"
        @click="emit('settings')"
      >
        Settings
      </button>
      <button
        class="btn btn-danger-outline"
        @click="emit('remove')"
        title="Remove from ServerMark (files are kept)"
      >
        Remove
      </button>
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
  gap: 8px;
  flex-wrap: wrap;
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
