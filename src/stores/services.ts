import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Service, ServiceStatus } from '@/types'

export const useServicesStore = defineStore('services', () => {
  const services = ref<Service[]>([
    { id: 'nginx', name: 'Nginx', status: 'stopped', port: 80 },
    { id: 'mysql', name: 'MySQL', status: 'stopped', port: 3306 },
    { id: 'postgresql', name: 'PostgreSQL', status: 'stopped', port: 5432 },
    { id: 'redis', name: 'Redis', status: 'stopped', port: 6379 },
    { id: 'mailpit', name: 'Mailpit', status: 'stopped', port: 8025 },
    { id: 'minio', name: 'MinIO', status: 'stopped', port: 9000 },
    { id: 'dnsmasq', name: 'DNSMasq', status: 'stopped', port: 53 },
  ])

  const runningCount = computed(() => services.value.filter((s) => s.status === 'running').length)

  const stoppedCount = computed(() => services.value.filter((s) => s.status === 'stopped').length)

  function updateStatus(id: string, status: ServiceStatus) {
    const service = services.value.find((s) => s.id === id)
    if (service) {
      service.status = status
    }
  }

  async function startService(id: string) {
    updateStatus(id, 'starting')
    // TODO: Call Tauri command
    // await invoke('start_service', { id })
    updateStatus(id, 'running')
  }

  async function stopService(id: string) {
    updateStatus(id, 'stopping')
    // TODO: Call Tauri command
    // await invoke('stop_service', { id })
    updateStatus(id, 'stopped')
  }

  async function restartService(id: string) {
    await stopService(id)
    await startService(id)
  }

  return {
    services,
    runningCount,
    stoppedCount,
    startService,
    stopService,
    restartService,
    updateStatus,
  }
})
