import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Site } from '@/types'

export const useSitesStore = defineStore('sites', () => {
  const sites = ref<Site[]>([])

  const activeSites = computed(() => sites.value.filter((s) => s.secured))

  function addSite(site: Omit<Site, 'id'>) {
    const id = `site-${Date.now()}`
    sites.value.push({ ...site, id })
  }

  function removeSite(id: string) {
    const index = sites.value.findIndex((s) => s.id === id)
    if (index !== -1) {
      sites.value.splice(index, 1)
    }
  }

  function updateSite(id: string, updates: Partial<Site>) {
    const site = sites.value.find((s) => s.id === id)
    if (site) {
      Object.assign(site, updates)
    }
  }

  async function secureSite(id: string) {
    // TODO: Call Tauri command to generate SSL certificate
    updateSite(id, { secured: true })
  }

  async function unsecureSite(id: string) {
    // TODO: Call Tauri command to remove SSL certificate
    updateSite(id, { secured: false })
  }

  return {
    sites,
    activeSites,
    addSite,
    removeSite,
    updateSite,
    secureSite,
    unsecureSite,
  }
})
