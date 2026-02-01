import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Site } from '@/types'

export type { Site }

export interface FrameworkTemplate {
  id: string
  name: string
  description: string
  versions: string[]
  default_version: string
}

export interface SitesConfig {
  sites: Site[]
  tld: string
  sites_path: string
}

export const useSitesStore = defineStore('sites', () => {
  // State
  const sites = ref<Site[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const frameworks = ref<FrameworkTemplate[]>([])
  const config = ref<SitesConfig>({
    sites: [],
    tld: 'test',
    sites_path: '',
  })

  // Getters
  const activeSites = computed(() => sites.value.filter((s) => s.secured))
  const siteCount = computed(() => sites.value.length)

  // Actions
  async function fetchSites(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      sites.value = await invoke<Site[]>('list_sites')
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch sites'
    } finally {
      loading.value = false
    }
  }

  async function fetchConfig(): Promise<void> {
    try {
      config.value = await invoke<SitesConfig>('get_sites_config')
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch config'
    }
  }

  async function fetchFrameworks(): Promise<void> {
    try {
      frameworks.value = await invoke<FrameworkTemplate[]>('get_framework_templates')
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch frameworks'
    }
  }

  async function addSite(path: string, name?: string, phpVersion?: string): Promise<Site> {
    loading.value = true
    error.value = null
    try {
      const site = await invoke<Site>('add_site', {
        path,
        name: name || null,
        php_version: phpVersion || null,
      })
      await fetchSites()
      return site
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to add site'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removeSite(id: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('remove_site', { id })
      await fetchSites()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to remove site'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateSitePhp(id: string, phpVersion: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('update_site_php', { id, phpVersion })
      await fetchSites()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update PHP version'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createProject(
    name: string,
    framework: string,
    version?: string,
    phpVersion?: string,
    path?: string
  ): Promise<Site> {
    loading.value = true
    error.value = null
    try {
      const site = await invoke<Site>('create_project', {
        name,
        framework,
        version: version || null,
        php_version: phpVersion || null,
        path: path || null,
      })
      await fetchSites()
      return site
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create project'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function cloneRepository(
    repoUrl: string,
    name?: string,
    phpVersion?: string
  ): Promise<Site> {
    loading.value = true
    error.value = null
    try {
      const site = await invoke<Site>('clone_repository', {
        repoUrl,
        name: name || null,
        phpVersion: phpVersion || null,
      })
      await fetchSites()
      return site
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to clone repository'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function secureSite(id: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('secure_site', { id })
      await fetchSites()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to secure site'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function unsecureSite(id: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('unsecure_site', { id })
      await fetchSites()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to unsecure site'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    // State
    sites,
    loading,
    error,
    frameworks,
    config,
    // Getters
    activeSites,
    siteCount,
    // Actions
    fetchSites,
    fetchConfig,
    fetchFrameworks,
    addSite,
    removeSite,
    updateSitePhp,
    createProject,
    cloneRepository,
    secureSite,
    unsecureSite,
  }
})
