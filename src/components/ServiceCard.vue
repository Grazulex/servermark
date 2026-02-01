<script setup lang="ts">
import { computed } from 'vue'
import type { Service } from '@/types'

const props = defineProps<{
  service: Service
}>()

const emit = defineEmits<{
  start: []
  stop: []
  restart: []
}>()

const statusClass = computed(() => props.service.status)

const isLoading = computed(() =>
  ['starting', 'stopping'].includes(props.service.status)
)

const canStart = computed(() => props.service.status === 'stopped')
const canStop = computed(() => props.service.status === 'running')
</script>

<template>
  <div
    class="service-card"
    :class="statusClass"
  >
    <div class="service-header">
      <div class="service-info">
        <div class="service-name">{{ service.name }}</div>
        <div class="service-port">Port {{ service.port }}</div>
      </div>
      <div
        class="service-status"
        :class="statusClass"
      >
        <span class="status-dot" />
        <span class="status-text">{{ service.status }}</span>
      </div>
    </div>

    <div class="service-actions">
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
        class="btn btn-danger"
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
    </div>
  </div>
</template>

<style scoped>
.service-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
  transition: all 0.2s ease;
}

.service-card:hover {
  border-color: var(--color-border-hover);
}

.service-card.running {
  border-left: 3px solid var(--color-success);
}

.service-card.stopped {
  border-left: 3px solid var(--color-text-muted);
}

.service-card.starting,
.service-card.stopping {
  border-left: 3px solid var(--color-warning);
}

.service-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.service-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.service-port {
  font-size: 13px;
  color: var(--color-text-muted);
  margin-top: 2px;
  font-family: var(--font-mono);
}

.service-status {
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

.service-status.running .status-dot {
  background: var(--color-success);
}

.service-status.stopped .status-dot {
  background: var(--color-text-muted);
}

.service-status.starting .status-dot,
.service-status.stopping .status-dot {
  background: var(--color-warning);
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.service-actions {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
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

.btn-danger {
  background: var(--color-danger);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: var(--color-danger-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-bg-hover);
}
</style>
