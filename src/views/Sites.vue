<script setup lang="ts">
import { useSitesStore } from '@/stores'
import SiteCard from '@/components/SiteCard.vue'

const sitesStore = useSitesStore()

function openSite(domain: string, secured: boolean) {
  const url = `http${secured ? 's' : ''}://${domain}`
  window.open(url, '_blank')
}
</script>

<template>
  <div class="sites-page">
    <header class="page-header">
      <div>
        <h1>Sites</h1>
        <p class="subtitle">Manage your local development sites</p>
      </div>
      <button class="btn btn-primary">
        + Add Site
      </button>
    </header>

    <div
      v-if="sitesStore.sites.length === 0"
      class="empty-state"
    >
      <div class="empty-icon">S</div>
      <h3>No sites configured</h3>
      <p>Add your first site to get started with local development.</p>
      <button class="btn btn-primary">
        + Add Site
      </button>
    </div>

    <div
      v-else
      class="sites-grid"
    >
      <SiteCard
        v-for="site in sitesStore.sites"
        :key="site.id"
        :site="site"
        @open="openSite(site.domain, site.secured)"
        @secure="sitesStore.secureSite(site.id)"
        @unsecure="sitesStore.unsecureSite(site.id)"
        @settings="() => {}"
        @upgrade-laravel="() => {}"
        @remove="sitesStore.removeSite(site.id)"
      />
    </div>
  </div>
</template>

<style scoped>
.sites-page {
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

.btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
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

.empty-state {
  text-align: center;
  padding: 60px 20px;
  background: var(--color-bg-secondary);
  border: 1px dashed var(--color-border);
  border-radius: 12px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 16px;
  background: var(--color-bg-tertiary);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-muted);
}

.empty-state h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 8px;
}

.empty-state p {
  color: var(--color-text-muted);
  margin: 0 0 20px;
}

.sites-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 16px;
}
</style>
