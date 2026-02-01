<script setup lang="ts">
import { computed } from 'vue'
import type { Container } from '@/types/docker'

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

    <div class="container-actions">
      <button
        v-if="canStart"
        class="btn btn-success"
        :disabled="isLoading"
        @click="emit('start')"
      >
        Start
      </button>
      <button
        v-if="canStop"
        class="btn btn-warning"
        :disabled="isLoading"
        @click="emit('stop')"
      >
        Stop
      </button>
      <button
        v-if="canStop"
        class="btn btn-secondary"
        :disabled="isLoading"
        @click="emit('restart')"
      >
        Restart
      </button>
      <button
        class="btn btn-secondary"
        @click="emit('logs')"
      >
        Logs
      </button>
      <button
        class="btn btn-danger-outline"
        :disabled="isLoading || container.status === 'running'"
        @click="emit('remove')"
      >
        Remove
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
</style>
