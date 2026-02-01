<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Container } from '@/types/docker'

interface ConnectionInfo {
  label: string
  value: string
  envKey?: string
}

const props = defineProps<{
  container: Container
}>()

const emit = defineEmits<{
  start: []
  stop: []
  restart: []
  remove: []
  logs: []
}>()

const showConnectionInfo = ref(false)
const copiedKey = ref<string | null>(null)

const statusClass = computed(() => props.container.status)

const isLoading = computed(() =>
  ['restarting', 'removing'].includes(props.container.status)
)

const canStart = computed(() => props.container.status === 'stopped')
const canStop = computed(() => props.container.status === 'running')

const portsDisplay = computed(() => {
  return props.container.ports
    .map((p) => `${p.host}:${p.container}`)
    .join(', ')
})

const serviceIcon = computed(() => {
  const name = props.container.name.replace('servermark-', '')
  const icons: Record<string, string> = {
    mysql: 'M',
    postgresql: 'P',
    postgres: 'P',
    redis: 'R',
    mailpit: 'E',
    minio: 'S',
    adminer: 'A',
    memcached: 'C',
    mongodb: 'M',
  }
  return icons[name] || name.charAt(0).toUpperCase()
})

const serviceName = computed(() => props.container.name.replace('servermark-', ''))

// Connection information based on service type
const connectionInfo = computed((): ConnectionInfo[] => {
  const name = serviceName.value
  const hostPort = props.container.ports[0]?.host || 0

  switch (name) {
    case 'mysql':
      return [
        { label: 'Host', value: '127.0.0.1', envKey: 'DB_HOST' },
        { label: 'Port', value: String(hostPort || 3306), envKey: 'DB_PORT' },
        { label: 'Database', value: 'servermark', envKey: 'DB_DATABASE' },
        { label: 'Username', value: 'root', envKey: 'DB_USERNAME' },
        { label: 'Password', value: 'secret', envKey: 'DB_PASSWORD' },
      ]
    case 'postgresql':
      return [
        { label: 'Host', value: '127.0.0.1', envKey: 'DB_HOST' },
        { label: 'Port', value: String(hostPort || 5432), envKey: 'DB_PORT' },
        { label: 'Database', value: 'servermark', envKey: 'DB_DATABASE' },
        { label: 'Username', value: 'postgres', envKey: 'DB_USERNAME' },
        { label: 'Password', value: 'secret', envKey: 'DB_PASSWORD' },
      ]
    case 'redis':
      return [
        { label: 'Host', value: '127.0.0.1', envKey: 'REDIS_HOST' },
        { label: 'Port', value: String(hostPort || 6379), envKey: 'REDIS_PORT' },
        { label: 'Password', value: 'null', envKey: 'REDIS_PASSWORD' },
      ]
    case 'mongodb':
      return [
        { label: 'Host', value: '127.0.0.1', envKey: 'MONGO_HOST' },
        { label: 'Port', value: String(hostPort || 27017), envKey: 'MONGO_PORT' },
        { label: 'Database', value: 'admin', envKey: 'MONGO_DATABASE' },
        { label: 'Username', value: 'root', envKey: 'MONGO_USERNAME' },
        { label: 'Password', value: 'secret', envKey: 'MONGO_PASSWORD' },
      ]
    case 'mailpit':
      return [
        { label: 'SMTP Host', value: '127.0.0.1', envKey: 'MAIL_HOST' },
        { label: 'SMTP Port', value: '1025', envKey: 'MAIL_PORT' },
        { label: 'Mailer', value: 'smtp', envKey: 'MAIL_MAILER' },
        { label: 'Web UI', value: `http://127.0.0.1:${props.container.ports[1]?.host || 8025}` },
      ]
    case 'minio':
      return [
        { label: 'Endpoint', value: `http://127.0.0.1:${hostPort || 9000}`, envKey: 'AWS_ENDPOINT' },
        { label: 'Access Key', value: 'minioadmin', envKey: 'AWS_ACCESS_KEY_ID' },
        { label: 'Secret Key', value: 'minioadmin', envKey: 'AWS_SECRET_ACCESS_KEY' },
        { label: 'Console', value: `http://127.0.0.1:${props.container.ports[1]?.host || 9001}` },
      ]
    case 'memcached':
      return [
        { label: 'Host', value: '127.0.0.1', envKey: 'MEMCACHED_HOST' },
        { label: 'Port', value: String(hostPort || 11211), envKey: 'MEMCACHED_PORT' },
      ]
    case 'adminer':
      return [
        { label: 'Web UI', value: `http://127.0.0.1:${hostPort || 8080}` },
      ]
    default:
      return []
  }
})

async function copyToClipboard(text: string, key: string) {
  try {
    await navigator.clipboard.writeText(text)
    copiedKey.value = key
    setTimeout(() => {
      copiedKey.value = null
    }, 2000)
  } catch (e) {
    console.error('Failed to copy:', e)
  }
}

async function copyAllEnv() {
  const envLines = connectionInfo.value
    .filter(info => info.envKey)
    .map(info => `${info.envKey}=${info.value}`)
    .join('\n')

  try {
    await navigator.clipboard.writeText(envLines)
    copiedKey.value = 'all'
    setTimeout(() => {
      copiedKey.value = null
    }, 2000)
  } catch (e) {
    console.error('Failed to copy:', e)
  }
}
</script>

<template>
  <div
    class="container-card"
    :class="statusClass"
  >
    <div class="container-header">
      <div class="container-icon">{{ serviceIcon }}</div>
      <div class="container-info">
        <div class="container-name">{{ container.name.replace('servermark-', '') }}</div>
        <div class="container-image">{{ container.image }}</div>
      </div>
      <div
        class="container-status"
        :class="statusClass"
      >
        <span class="status-dot" />
        <span class="status-text">{{ container.status }}</span>
      </div>
    </div>

    <div
      v-if="container.ports.length > 0"
      class="container-ports"
    >
      <span class="ports-label">Ports:</span>
      <span class="ports-value">{{ portsDisplay }}</span>
    </div>

    <!-- Connection Info Toggle -->
    <button
      v-if="connectionInfo.length > 0"
      class="connection-toggle"
      @click="showConnectionInfo = !showConnectionInfo"
    >
      <span class="toggle-icon">{{ showConnectionInfo ? '▼' : '▶' }}</span>
      Connection Info
    </button>

    <!-- Connection Details -->
    <div
      v-if="showConnectionInfo && connectionInfo.length > 0"
      class="connection-info"
    >
      <div
        v-for="info in connectionInfo"
        :key="info.label"
        class="connection-row"
        @click="copyToClipboard(info.value, info.label)"
      >
        <span class="connection-label">{{ info.label }}</span>
        <span class="connection-value">
          <code>{{ info.value }}</code>
          <span
            v-if="copiedKey === info.label"
            class="copied-badge"
          >Copied!</span>
        </span>
      </div>
      <button
        v-if="connectionInfo.some(i => i.envKey)"
        class="copy-env-btn"
        @click.stop="copyAllEnv"
      >
        {{ copiedKey === 'all' ? 'Copied!' : 'Copy .env variables' }}
      </button>
    </div>

    <div class="container-actions">
      <button
        v-if="canStart"
        class="btn btn-icon btn-success"
        :disabled="isLoading"
        title="Start"
        @click="emit('start')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" stroke="none">
          <polygon points="5 3 19 12 5 21 5 3"/>
        </svg>
      </button>
      <button
        v-if="canStop"
        class="btn btn-icon btn-warning"
        :disabled="isLoading"
        title="Stop"
        @click="emit('stop')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" stroke="none">
          <rect x="4" y="4" width="16" height="16" rx="2"/>
        </svg>
      </button>
      <button
        v-if="canStop"
        class="btn btn-icon btn-secondary"
        :disabled="isLoading"
        title="Restart"
        @click="emit('restart')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="23 4 23 10 17 10"/>
          <polyline points="1 20 1 14 7 14"/>
          <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
        </svg>
      </button>
      <button
        class="btn btn-icon btn-secondary"
        title="Logs"
        @click="emit('logs')"
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
        class="btn btn-icon btn-danger-outline"
        :disabled="isLoading || container.status === 'running'"
        title="Remove"
        @click="emit('remove')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"/>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.container-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
  transition: all 0.2s ease;
}

.container-card:hover {
  border-color: var(--color-border-hover);
}

.container-card.running {
  border-left: 3px solid var(--color-success);
}

.container-card.stopped {
  border-left: 3px solid var(--color-text-muted);
}

.container-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.container-icon {
  width: 40px;
  height: 40px;
  background: var(--color-bg-tertiary);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-primary);
}

.container-info {
  flex: 1;
}

.container-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-transform: capitalize;
}

.container-image {
  font-size: 12px;
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

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.container-status.running .status-dot {
  background: var(--color-success);
}

.container-status.stopped .status-dot {
  background: var(--color-text-muted);
}

.container-status.running {
  color: var(--color-success);
}

.container-status.stopped {
  color: var(--color-text-muted);
}

.container-ports {
  font-size: 13px;
  padding: 8px 12px;
  background: var(--color-bg-tertiary);
  border-radius: 6px;
  margin-bottom: 16px;
  font-family: var(--font-mono);
}

.ports-label {
  color: var(--color-text-muted);
  margin-right: 8px;
}

.ports-value {
  color: var(--color-text-primary);
}

.container-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: flex-end;
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

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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

.btn-success {
  background: var(--color-success);
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: var(--color-success-hover);
}

.btn-warning {
  background: var(--color-warning);
  color: white;
}

.btn-warning:hover:not(:disabled) {
  background: var(--color-warning-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-bg-hover);
}

.btn-danger-outline {
  background: transparent;
  border: 1px solid var(--color-danger);
  color: var(--color-danger);
}

.btn-danger-outline:hover:not(:disabled) {
  background: var(--color-danger);
  color: white;
}

/* Connection Info */
.connection-toggle {
  width: 100%;
  padding: 8px 12px;
  background: var(--color-bg-tertiary);
  border: none;
  border-radius: 6px;
  color: var(--color-text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  transition: all 0.15s ease;
}

.connection-toggle:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.toggle-icon {
  font-size: 10px;
  color: var(--color-text-muted);
}

.connection-info {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 16px;
  font-size: 12px;
}

.connection-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s ease;
}

.connection-row:hover {
  background: var(--color-bg-tertiary);
}

.connection-label {
  color: var(--color-text-muted);
  font-weight: 500;
}

.connection-value {
  display: flex;
  align-items: center;
  gap: 8px;
}

.connection-value code {
  font-family: var(--font-mono);
  color: var(--color-primary);
  background: var(--color-bg-tertiary);
  padding: 2px 6px;
  border-radius: 4px;
}

.copied-badge {
  font-size: 10px;
  color: var(--color-success);
  font-weight: 600;
}

.copy-env-btn {
  width: 100%;
  margin-top: 12px;
  padding: 8px;
  background: var(--color-primary);
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.15s ease;
}

.copy-env-btn:hover {
  background: var(--color-primary-hover);
}
</style>
