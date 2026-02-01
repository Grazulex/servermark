<script setup lang="ts">
import { computed } from 'vue'
import type { ServiceStatus } from '@/types'

const props = defineProps<{
  status: ServiceStatus
  size?: 'sm' | 'md' | 'lg'
}>()

const sizeClass = computed(() => props.size ?? 'md')
</script>

<template>
  <span
    class="status-badge"
    :class="[status, sizeClass]"
  >
    <span class="dot" />
    <span class="label">{{ status }}</span>
  </span>
</template>

<style scoped>
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 100px;
  font-weight: 500;
  text-transform: capitalize;
}

.status-badge.sm {
  font-size: 11px;
  padding: 2px 8px;
}

.status-badge.md {
  font-size: 12px;
}

.status-badge.lg {
  font-size: 14px;
  padding: 6px 14px;
}

.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.status-badge.running {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.status-badge.running .dot {
  background: var(--color-success);
}

.status-badge.stopped {
  background: rgba(107, 114, 128, 0.1);
  color: var(--color-text-muted);
}

.status-badge.stopped .dot {
  background: var(--color-text-muted);
}

.status-badge.starting,
.status-badge.stopping {
  background: rgba(245, 158, 11, 0.1);
  color: var(--color-warning);
}

.status-badge.starting .dot,
.status-badge.stopping .dot {
  background: var(--color-warning);
  animation: pulse 1s infinite;
}

.status-badge.error {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-danger);
}

.status-badge.error .dot {
  background: var(--color-danger);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>
