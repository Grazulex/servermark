<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDockerStore } from '@/stores/docker'

interface Database {
  name: string
  size: string | null
}

const dockerStore = useDockerStore()

const databases = ref<Database[]>([])
const loading = ref(false)
const creating = ref(false)
const deleting = ref<string | null>(null)

// Form
const showCreateModal = ref(false)
const newDbName = ref('')
const selectedDbType = ref<'mysql' | 'postgresql'>('mysql')

// Confirmation modal
const showDeleteConfirm = ref(false)
const dbToDelete = ref<Database | null>(null)

const mysqlContainer = computed(() => {
  return dockerStore.containers.find(c =>
    c.name.includes('mysql') || c.name.includes('mariadb')
  )
})

const postgresContainer = computed(() => {
  return dockerStore.containers.find(c =>
    c.name.includes('postgres')
  )
})

const availableDbTypes = computed(() => {
  const types: { value: 'mysql' | 'postgresql'; label: string; available: boolean; containerId?: string }[] = []

  if (mysqlContainer.value?.status === 'running') {
    types.push({
      value: 'mysql',
      label: 'MySQL/MariaDB (Container)',
      available: true,
      containerId: mysqlContainer.value.id
    })
  }

  if (postgresContainer.value?.status === 'running') {
    types.push({
      value: 'postgresql',
      label: 'PostgreSQL (Container)',
      available: true,
      containerId: postgresContainer.value.id
    })
  }

  return types
})

async function loadDatabases() {
  if (availableDbTypes.value.length === 0) {
    databases.value = []
    return
  }

  loading.value = true
  databases.value = []

  try {
    for (const dbType of availableDbTypes.value) {
      if (dbType.available) {
        const dbs = await invoke<Database[]>('list_databases', {
          dbType: dbType.value,
          containerId: dbType.containerId || null
        })
        databases.value.push(...dbs.map(db => ({
          ...db,
          type: dbType.value,
          containerId: dbType.containerId
        } as any)))
      }
    }
  } catch (e) {
    console.error('Failed to load databases:', e)
  } finally {
    loading.value = false
  }
}

async function createDatabase() {
  if (!newDbName.value.trim()) return

  const dbType = availableDbTypes.value.find(t => t.value === selectedDbType.value)
  if (!dbType) return

  creating.value = true
  try {
    await invoke('create_database', {
      dbType: selectedDbType.value,
      name: newDbName.value.trim(),
      containerId: dbType.containerId || null
    })
    showCreateModal.value = false
    newDbName.value = ''
    await loadDatabases()
  } catch (e) {
    console.error('Failed to create database:', e)
    alert(`Failed to create database: ${e}`)
  } finally {
    creating.value = false
  }
}

function confirmDelete(db: Database) {
  dbToDelete.value = db
  showDeleteConfirm.value = true
}

async function deleteDatabase() {
  if (!dbToDelete.value) return

  const db = dbToDelete.value as any
  deleting.value = db.name

  try {
    await invoke('drop_database', {
      dbType: db.type,
      name: db.name,
      containerId: db.containerId || null
    })
    showDeleteConfirm.value = false
    dbToDelete.value = null
    await loadDatabases()
  } catch (e) {
    console.error('Failed to delete database:', e)
    alert(`Failed to delete database: ${e}`)
  } finally {
    deleting.value = null
  }
}

onMounted(async () => {
  await dockerStore.listContainers()
  await loadDatabases()
})
</script>

<template>
  <div class="databases-page">
    <header class="page-header">
      <div>
        <h1>Databases</h1>
        <p class="subtitle">Manage your MySQL and PostgreSQL databases</p>
      </div>
      <button
        class="btn btn-primary"
        :disabled="availableDbTypes.length === 0"
        @click="showCreateModal = true"
      >
        + Create Database
      </button>
    </header>

    <!-- No database containers -->
    <div v-if="availableDbTypes.length === 0" class="empty-state">
      <div class="empty-icon">DB</div>
      <h3>No Database Containers Running</h3>
      <p>Start a MySQL, MariaDB, or PostgreSQL container to manage databases.</p>
      <router-link to="/containers" class="btn btn-secondary">
        Go to Containers
      </router-link>
    </div>

    <!-- Database List -->
    <div v-else class="databases-content">
      <!-- Available Connections -->
      <div class="connections-bar">
        <span class="connections-label">Connected to:</span>
        <span
          v-for="dbType in availableDbTypes"
          :key="dbType.value"
          class="connection-badge"
        >
          {{ dbType.label }}
        </span>
      </div>

      <!-- Loading -->
      <div v-if="loading" class="loading-state">
        Loading databases...
      </div>

      <!-- Database Grid -->
      <div v-else-if="databases.length > 0" class="databases-grid">
        <div
          v-for="db in databases"
          :key="`${(db as any).type}-${db.name}`"
          class="database-card"
        >
          <div class="db-header">
            <div class="db-icon">
              {{ (db as any).type === 'mysql' ? 'M' : 'P' }}
            </div>
            <div class="db-info">
              <div class="db-name">{{ db.name }}</div>
              <div class="db-type">{{ (db as any).type === 'mysql' ? 'MySQL' : 'PostgreSQL' }}</div>
            </div>
          </div>
          <div class="db-actions">
            <button
              class="btn btn-danger-outline btn-sm"
              :disabled="deleting === db.name"
              @click="confirmDelete(db)"
            >
              {{ deleting === db.name ? 'Deleting...' : 'Drop' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Empty -->
      <div v-else class="empty-databases">
        <p>No user databases found. Create your first database!</p>
      </div>
    </div>

    <!-- Create Modal -->
    <Teleport to="body">
      <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
        <div class="modal">
          <div class="modal-header">
            <h3>Create Database</h3>
            <button class="close-btn" @click="showCreateModal = false">&times;</button>
          </div>
          <div class="modal-body">
            <div class="form-group">
              <label>Database Type</label>
              <select v-model="selectedDbType" class="form-select">
                <option
                  v-for="dbType in availableDbTypes"
                  :key="dbType.value"
                  :value="dbType.value"
                >
                  {{ dbType.label }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label>Database Name</label>
              <input
                v-model="newDbName"
                type="text"
                class="form-input"
                placeholder="my_database"
                pattern="[a-zA-Z0-9_]+"
                @keyup.enter="createDatabase"
              />
              <p class="form-help">Letters, numbers, and underscores only</p>
            </div>
          </div>
          <div class="modal-footer">
            <button class="btn btn-secondary" @click="showCreateModal = false">Cancel</button>
            <button
              class="btn btn-primary"
              :disabled="creating || !newDbName.trim()"
              @click="createDatabase"
            >
              {{ creating ? 'Creating...' : 'Create' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Delete Confirmation Modal -->
    <Teleport to="body">
      <div v-if="showDeleteConfirm" class="modal-overlay" @click.self="showDeleteConfirm = false">
        <div class="modal modal-sm">
          <div class="modal-header">
            <h3>Confirm Delete</h3>
            <button class="close-btn" @click="showDeleteConfirm = false">&times;</button>
          </div>
          <div class="modal-body">
            <p class="delete-warning">
              Are you sure you want to drop the database
              <strong>{{ dbToDelete?.name }}</strong>?
            </p>
            <p class="delete-note">This action cannot be undone.</p>
          </div>
          <div class="modal-footer">
            <button class="btn btn-secondary" @click="showDeleteConfirm = false">Cancel</button>
            <button
              class="btn btn-danger"
              :disabled="!!deleting"
              @click="deleteDatabase"
            >
              {{ deleting ? 'Deleting...' : 'Drop Database' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.databases-page {
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

/* Empty State */
.empty-state {
  text-align: center;
  padding: 60px 20px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
}

.empty-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 20px;
  background: var(--color-bg-tertiary);
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-primary);
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

/* Connections Bar */
.connections-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 10px;
  margin-bottom: 24px;
}

.connections-label {
  font-size: 13px;
  color: var(--color-text-muted);
}

.connection-badge {
  padding: 4px 12px;
  background: var(--color-success);
  color: white;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

/* Loading */
.loading-state {
  text-align: center;
  padding: 40px;
  color: var(--color-text-muted);
}

/* Empty Databases */
.empty-databases {
  text-align: center;
  padding: 40px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  color: var(--color-text-muted);
}

/* Database Grid */
.databases-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.database-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.db-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.db-icon {
  width: 44px;
  height: 44px;
  background: var(--color-bg-tertiary);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  color: var(--color-primary);
}

.db-info {
  flex: 1;
  min-width: 0;
}

.db-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  word-break: break-all;
}

.db-type {
  font-size: 12px;
  color: var(--color-text-muted);
}

.db-actions {
  display: flex;
  gap: 8px;
}

/* Buttons */
.btn {
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
  text-decoration: none;
  display: inline-block;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-secondary {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-bg-hover);
}

.btn-danger {
  background: var(--color-danger);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #dc2626;
}

.btn-danger-outline {
  background: transparent;
  border: 1px solid var(--color-danger);
  color: var(--color-danger);
}

.btn-danger-outline:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 16px;
  width: 90%;
  max-width: 480px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-sm {
  max-width: 400px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-bg-tertiary);
  border: none;
  border-radius: 8px;
  font-size: 20px;
  color: var(--color-text-muted);
  cursor: pointer;
}

.close-btn:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 20px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--color-border);
}

/* Form */
.form-group {
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
  margin-bottom: 6px;
}

.form-input,
.form-select {
  width: 100%;
  padding: 10px 14px;
  background: #1e1e2e;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 14px;
  color: #cdd6f4;
  font-family: var(--font-mono);
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
}

.form-select {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%239ca3af' d='M2 4l4 4 4-4'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 36px;
  cursor: pointer;
}

.form-select option {
  background: #1e1e2e;
  color: #cdd6f4;
  padding: 8px;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.form-help {
  font-size: 12px;
  color: var(--color-text-muted);
  margin: 6px 0 0;
}

/* Delete Warning */
.delete-warning {
  color: var(--color-text-primary);
  margin: 0 0 12px;
}

.delete-warning strong {
  color: var(--color-danger);
}

.delete-note {
  color: var(--color-text-muted);
  font-size: 13px;
  margin: 0;
}
</style>
