import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSitesStore } from '../sites'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('Sites Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('should have correct initial state', () => {
    const store = useSitesStore()

    expect(store.sites).toEqual([])
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
    expect(store.siteCount).toBe(0)
  })

  it('should compute siteCount correctly', () => {
    const store = useSitesStore()

    // Manually set sites for testing computed
    store.sites = [
      {
        id: '1',
        name: 'test-site',
        path: '/var/www/test',
        domain: 'test.test',
        php_version: '8.3',
        secured: false,
        site_type: 'static',
      },
      {
        id: '2',
        name: 'test-site-2',
        path: '/var/www/test2',
        domain: 'test2.test',
        php_version: '8.2',
        secured: true,
        site_type: 'laravel',
      },
    ]

    expect(store.siteCount).toBe(2)
  })

  it('should compute activeSites (secured) correctly', () => {
    const store = useSitesStore()

    store.sites = [
      {
        id: '1',
        name: 'unsecured',
        path: '/var/www/test',
        domain: 'test.test',
        php_version: '8.3',
        secured: false,
        site_type: 'static',
      },
      {
        id: '2',
        name: 'secured',
        path: '/var/www/test2',
        domain: 'test2.test',
        php_version: '8.2',
        secured: true,
        site_type: 'laravel',
      },
    ]

    expect(store.activeSites).toHaveLength(1)
    expect(store.activeSites[0].name).toBe('secured')
  })

  it('should have default config values', () => {
    const store = useSitesStore()

    expect(store.config.tld).toBe('test')
    expect(store.config.sites).toEqual([])
  })
})
