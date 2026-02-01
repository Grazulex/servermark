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
      const fetchedSites = await invoke<Site[]>('list_sites')

      // Fetch scheduler and queue status for Laravel sites
      for (const site of fetchedSites) {
        if (site.site_type === 'laravel' && site.laravel) {
          try {
            const [schedulerEnabled, queueEnabled] = await Promise.all([
              invoke<boolean>('get_scheduler_status', { sitePath: site.path }),
              invoke<boolean>('get_queue_status', { sitePath: site.path }),
            ])
            site.laravel.scheduler_enabled = schedulerEnabled
            site.laravel.queue_enabled = queueEnabled
          } catch {
            site.laravel.scheduler_enabled = false
            site.laravel.queue_enabled = false
          }
        }
      }

      sites.value = fetchedSites
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

  async function getSchedulerStatus(sitePath: string): Promise<boolean> {
    try {
      return await invoke<boolean>('get_scheduler_status', { sitePath })
    } catch (e) {
      console.error('Failed to get scheduler status:', e)
      return false
    }
  }

  async function enableScheduler(site: Site): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('enable_scheduler', {
        sitePath: site.path,
        phpVersion: site.php_version,
      })
      // Update local state
      const siteIndex = sites.value.findIndex(s => s.id === site.id)
      if (siteIndex !== -1 && sites.value[siteIndex].laravel) {
        sites.value[siteIndex].laravel!.scheduler_enabled = true
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to enable scheduler'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function disableScheduler(site: Site): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('disable_scheduler', { sitePath: site.path })
      // Update local state
      const siteIndex = sites.value.findIndex(s => s.id === site.id)
      if (siteIndex !== -1 && sites.value[siteIndex].laravel) {
        sites.value[siteIndex].laravel!.scheduler_enabled = false
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to disable scheduler'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function toggleScheduler(site: Site): Promise<void> {
    if (site.laravel?.scheduler_enabled) {
      await disableScheduler(site)
    } else {
      await enableScheduler(site)
    }
  }

  async function getQueueStatus(sitePath: string): Promise<boolean> {
    try {
      return await invoke<boolean>('get_queue_status', { sitePath })
    } catch (e) {
      console.error('Failed to get queue status:', e)
      return false
    }
  }

  async function startQueueWorker(site: Site): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('start_queue_worker', {
        sitePath: site.path,
        phpVersion: site.php_version,
        siteName: site.name,
      })
      // Update local state
      const siteIndex = sites.value.findIndex(s => s.id === site.id)
      if (siteIndex !== -1 && sites.value[siteIndex].laravel) {
        sites.value[siteIndex].laravel!.queue_enabled = true
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to start queue worker'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function stopQueueWorker(site: Site): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('stop_queue_worker', { sitePath: site.path })
      // Update local state
      const siteIndex = sites.value.findIndex(s => s.id === site.id)
      if (siteIndex !== -1 && sites.value[siteIndex].laravel) {
        sites.value[siteIndex].laravel!.queue_enabled = false
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to stop queue worker'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function toggleQueueWorker(site: Site): Promise<void> {
    if (site.laravel?.queue_enabled) {
      await stopQueueWorker(site)
    } else {
      await startQueueWorker(site)
    }
  }

  async function getSchedulerLogs(sitePath: string, lines?: number): Promise<string> {
    try {
      return await invoke<string>('get_scheduler_logs', { sitePath, lines })
    } catch (e) {
      return `Error fetching scheduler logs: ${e}`
    }
  }

  async function getQueueLogs(sitePath: string, lines?: number): Promise<string> {
    try {
      return await invoke<string>('get_queue_logs', { sitePath, lines })
    } catch (e) {
      return `Error fetching queue logs: ${e}`
    }
  }

  async function clearSchedulerLogs(sitePath: string): Promise<void> {
    await invoke('clear_scheduler_logs', { sitePath })
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
    getSchedulerStatus,
    enableScheduler,
    disableScheduler,
    toggleScheduler,
    getQueueStatus,
    startQueueWorker,
    stopQueueWorker,
    toggleQueueWorker,
    getSchedulerLogs,
    getQueueLogs,
    clearSchedulerLogs,
  }
})
