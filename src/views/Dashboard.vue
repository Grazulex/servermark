<script setup lang="ts">
import { onMounted } from 'vue'
import { useServicesStore, usePhpStore, useSitesStore } from '@/stores'
import ServiceCard from '@/components/ServiceCard.vue'

const servicesStore = useServicesStore()
const phpStore = usePhpStore()
const sitesStore = useSitesStore()

onMounted(async () => {
  // Load PHP versions if not already loaded
  if (phpStore.versions.length === 0) {
    await phpStore.fetchVersions()
  }
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
        <div class="stat-value">{{ servicesStore.runningCount }}</div>
        <div class="stat-label">Services Running</div>
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

    <!-- Services Overview -->
    <section class="section">
      <div class="section-header">
        <h2>Services</h2>
        <router-link
          to="/services"
          class="link"
        >
          View all
        </router-link>
      </div>
      <div class="services-grid">
        <ServiceCard
          v-for="service in servicesStore.services.slice(0, 4)"
          :key="service.id"
          :service="service"
          @start="servicesStore.startService(service.id)"
          @stop="servicesStore.stopService(service.id)"
          @restart="servicesStore.restartService(service.id)"
        />
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
</style>
