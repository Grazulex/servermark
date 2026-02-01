<script setup lang="ts">
import { useServicesStore } from '@/stores'
import ServiceCard from '@/components/ServiceCard.vue'

const servicesStore = useServicesStore()

async function startAll() {
  for (const service of servicesStore.services) {
    if (service.status === 'stopped') {
      await servicesStore.startService(service.id)
    }
  }
}

async function stopAll() {
  for (const service of servicesStore.services) {
    if (service.status === 'running') {
      await servicesStore.stopService(service.id)
    }
  }
}
</script>

<template>
  <div class="services-page">
    <header class="page-header">
      <div>
        <h1>Services</h1>
        <p class="subtitle">Manage your local development services</p>
      </div>
      <div class="header-actions">
        <button
          class="btn btn-success"
          @click="startAll"
        >
          Start All
        </button>
        <button
          class="btn btn-danger"
          @click="stopAll"
        >
          Stop All
        </button>
      </div>
    </header>

    <div class="services-grid">
      <ServiceCard
        v-for="service in servicesStore.services"
        :key="service.id"
        :service="service"
        @start="servicesStore.startService(service.id)"
        @stop="servicesStore.stopService(service.id)"
        @restart="servicesStore.restartService(service.id)"
      />
    </div>
  </div>
</template>

<style scoped>
.services-page {
  max-width: 1200px;
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
}

.header-actions {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-success {
  background: var(--color-success);
  color: white;
}

.btn-success:hover {
  background: var(--color-success-hover);
}

.btn-danger {
  background: var(--color-danger);
  color: white;
}

.btn-danger:hover {
  background: var(--color-danger-hover);
}

.services-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;
}
</style>
