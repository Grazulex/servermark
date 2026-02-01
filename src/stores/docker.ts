import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  Container,
  RuntimeInfo,
  ServiceTemplate,
  ServiceType,
  PortMapping,
} from '@/types/docker'

// Service templates
const serviceTemplates: ServiceTemplate[] = [
  {
    id: 'mysql',
    name: 'MySQL',
    description: 'Popular open-source relational database',
    image: 'mysql',
    defaultTag: '8.0',
    availableTags: ['8.0', '8.4', '5.7'],
    ports: [{ host: 3306, container: 3306, protocol: 'tcp' }],
    environment: {
      MYSQL_ROOT_PASSWORD: 'secret',
      MYSQL_DATABASE: 'servermark',
    },
    volumes: [{ name: 'mysql_data', container: '/var/lib/mysql', description: 'Database files' }],
    category: 'database',
  },
  {
    id: 'postgresql',
    name: 'PostgreSQL',
    description: 'Advanced open-source relational database',
    image: 'postgres',
    defaultTag: '16',
    availableTags: ['16', '15', '14', '13'],
    ports: [{ host: 5432, container: 5432, protocol: 'tcp' }],
    environment: {
      POSTGRES_PASSWORD: 'secret',
      POSTGRES_DB: 'servermark',
    },
    volumes: [
      { name: 'postgres_data', container: '/var/lib/postgresql/data', description: 'Database files' },
    ],
    category: 'database',
  },
  {
    id: 'redis',
    name: 'Redis',
    description: 'In-memory data structure store',
    image: 'redis',
    defaultTag: '7-alpine',
    availableTags: ['7-alpine', '7', '6-alpine', '6'],
    ports: [{ host: 6379, container: 6379, protocol: 'tcp' }],
    environment: {},
    volumes: [{ name: 'redis_data', container: '/data', description: 'Persistent data' }],
    category: 'cache',
  },
  {
    id: 'mailpit',
    name: 'Mailpit',
    description: 'Email testing tool with web UI',
    image: 'axllent/mailpit',
    defaultTag: 'latest',
    availableTags: ['latest'],
    ports: [
      { host: 1025, container: 1025, protocol: 'tcp' },
      { host: 8025, container: 8025, protocol: 'tcp' },
    ],
    environment: {},
    category: 'mail',
  },
  {
    id: 'minio',
    name: 'MinIO',
    description: 'S3-compatible object storage',
    image: 'minio/minio',
    defaultTag: 'latest',
    availableTags: ['latest'],
    ports: [
      { host: 9000, container: 9000, protocol: 'tcp' },
      { host: 9001, container: 9001, protocol: 'tcp' },
    ],
    environment: {
      MINIO_ROOT_USER: 'minioadmin',
      MINIO_ROOT_PASSWORD: 'minioadmin',
    },
    volumes: [{ name: 'minio_data', container: '/data', description: 'Object storage' }],
    category: 'storage',
  },
  {
    id: 'adminer',
    name: 'Adminer',
    description: 'Database management in single PHP file',
    image: 'adminer',
    defaultTag: 'latest',
    availableTags: ['latest'],
    ports: [{ host: 8080, container: 8080, protocol: 'tcp' }],
    environment: {},
    category: 'tools',
  },
  {
    id: 'memcached',
    name: 'Memcached',
    description: 'Distributed memory caching system',
    image: 'memcached',
    defaultTag: 'alpine',
    availableTags: ['alpine', 'latest'],
    ports: [{ host: 11211, container: 11211, protocol: 'tcp' }],
    environment: {},
    category: 'cache',
  },
  {
    id: 'mongodb',
    name: 'MongoDB',
    description: 'NoSQL document database',
    image: 'mongo',
    defaultTag: '7',
    availableTags: ['7', '6', '5'],
    ports: [{ host: 27017, container: 27017, protocol: 'tcp' }],
    environment: {
      MONGO_INITDB_ROOT_USERNAME: 'root',
      MONGO_INITDB_ROOT_PASSWORD: 'secret',
    },
    volumes: [{ name: 'mongo_data', container: '/data/db', description: 'Database files' }],
    category: 'database',
  },
]

export const useDockerStore = defineStore('docker', () => {
  // State
  const runtime = ref<RuntimeInfo>({
    runtime: 'none',
    version: '',
    apiVersion: '',
    available: false,
  })

  const containers = ref<Container[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isAvailable = computed(() => runtime.value.available)

  const runningContainers = computed(() =>
    containers.value.filter((c) => c.status === 'running')
  )

  const stoppedContainers = computed(() =>
    containers.value.filter((c) => c.status === 'stopped')
  )

  const templates = computed(() => serviceTemplates)

  const templatesByCategory = computed(() => {
    const grouped: Record<string, ServiceTemplate[]> = {}
    for (const template of serviceTemplates) {
      const category = template.category
      if (!grouped[category]) {
        grouped[category] = []
      }
      grouped[category]!.push(template)
    }
    return grouped
  })

  // Actions
  async function detectRuntime(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const info = await invoke<RuntimeInfo>('detect_container_runtime')
      runtime.value = {
        runtime: info.runtime,
        version: info.version,
        apiVersion: info.apiVersion || info.api_version || '',
        available: info.available,
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to detect runtime'
      runtime.value = { runtime: 'none', version: '', apiVersion: '', available: false }
    } finally {
      loading.value = false
    }
  }

  async function listContainers(): Promise<void> {
    if (!runtime.value.available) return

    loading.value = true
    error.value = null
    try {
      containers.value = await invoke<Container[]>('list_containers')
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to list containers'
    } finally {
      loading.value = false
    }
  }

  async function createContainer(
    service: ServiceType,
    tag?: string,
    customPorts?: Record<number, number>
  ): Promise<void> {
    const template = serviceTemplates.find((t) => t.id === service)
    if (!template) throw new Error(`Unknown service: ${service}`)

    loading.value = true
    error.value = null
    try {
      // Build ports array
      const ports: PortMapping[] = template.ports.map((p) => ({
        host: customPorts?.[p.container] || p.host,
        container: p.container,
        protocol: p.protocol,
      }))

      // Build volumes array
      const volumes = template.volumes?.map((v) => ({
        name: `servermark-${service}-${v.name}`,
        container: v.container,
      }))

      await invoke('create_container', {
        params: {
          image: `${template.image}:${tag || template.defaultTag}`,
          name: `servermark-${service}`,
          ports,
          environment: template.environment,
          volumes,
        },
      })
      await listContainers()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create container'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function startContainer(id: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('start_container', { id })
      await listContainers()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to start container'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function stopContainer(id: string): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('stop_container', { id })
      await listContainers()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to stop container'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removeContainer(id: string, force: boolean = false): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await invoke('remove_container', { id, force })
      await listContainers()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to remove container'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function restartContainer(id: string): Promise<void> {
    await stopContainer(id)
    await startContainer(id)
  }

  async function getContainerLogs(id: string, lines: number = 100): Promise<string> {
    try {
      return await invoke<string>('get_container_logs', { id, lines })
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to get logs'
      throw e
    }
  }

  function getTemplate(service: ServiceType): ServiceTemplate | undefined {
    return serviceTemplates.find((t) => t.id === service)
  }

  return {
    // State
    runtime,
    containers,
    loading,
    error,
    // Getters
    isAvailable,
    runningContainers,
    stoppedContainers,
    templates,
    templatesByCategory,
    // Actions
    detectRuntime,
    listContainers,
    createContainer,
    startContainer,
    stopContainer,
    removeContainer,
    restartContainer,
    getContainerLogs,
    getTemplate,
  }
})
