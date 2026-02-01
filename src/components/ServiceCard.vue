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
  justify-content: flex-end;
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
</style>
