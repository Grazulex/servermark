import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useDockerStore } from '@/stores/docker'

export function useTrayEvents() {
  const dockerStore = useDockerStore()
  const unlisteners: UnlistenFn[] = []

  async function setupListeners() {
    // Listen for "Start All" from tray
    const unlistenStart = await listen('tray-start-all', async () => {
      console.log('Tray: Start all containers')
      for (const container of dockerStore.containers) {
        if (container.status === 'stopped') {
          await dockerStore.startContainer(container.id)
        }
      }
    })
    unlisteners.push(unlistenStart)

    // Listen for "Stop All" from tray
    const unlistenStop = await listen('tray-stop-all', async () => {
      console.log('Tray: Stop all containers')
      for (const container of dockerStore.containers) {
        if (container.status === 'running') {
          await dockerStore.stopContainer(container.id)
        }
      }
    })
    unlisteners.push(unlistenStop)
  }

  onMounted(() => {
    setupListeners()
  })

  onUnmounted(() => {
    unlisteners.forEach((unlisten) => unlisten())
  })
}
