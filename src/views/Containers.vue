<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useDockerStore } from '@/stores/docker'
import ContainerCard from '@/components/ContainerCard.vue'
import type { ServiceType, ServiceTemplate } from '@/types/docker'

const dockerStore = useDockerStore()

const showAddModal = ref(false)
const selectedService = ref<ServiceTemplate | null>(null)
const selectedTag = ref('')

onMounted(async () => {
  await dockerStore.detectRuntime()
  if (dockerStore.isAvailable) {
    await dockerStore.listContainers()
  }
})

const categoryNames: Record<string, string> = {
  database: 'Databases',
  cache: 'Cache',
  mail: 'Mail',
  storage: 'Storage',
  tools: 'Tools',
}

function selectService(template: ServiceTemplate) {
  selectedService.value = template
  selectedTag.value = template.defaultTag
}

async function createService() {
  if (!selectedService.value) return

  try {
    await dockerStore.createContainer(
      selectedService.value.id as ServiceType,
      selectedTag.value
    )
    showAddModal.value = false
    selectedService.value = null
  } catch (e) {
    console.error('Failed to create container:', e)
  }
}

function closeModal() {
  showAddModal.value = false
  selectedService.value = null
}
</script>

<template>
  <div class="containers-page">
    <header class="page-header">
      <div>
        <h1>Containers</h1>
        <p class="subtitle">
          <template v-if="dockerStore.isAvailable">
            {{ dockerStore.runtime.runtime }} {{ dockerStore.runtime.version }}
          </template>
          <template v-else>
            <span class="warning">No container runtime detected</span>
          </template>
        </p>
      </div>
      <button
        class="btn btn-primary"
        :disabled="!dockerStore.isAvailable"
        @click="showAddModal = true"
      >
        + Add Service
      </button>
    </header>

    <!-- No Runtime Warning -->
    <div
      v-if="!dockerStore.isAvailable"
      class="no-runtime"
    >
      <div class="no-runtime-icon">D</div>
      <h3>Container Runtime Required</h3>
      <p>ServerMark uses Docker or Podman to run services like MySQL, Redis, etc.</p>
      <div class="install-options">
        <a
          href="https://docs.docker.com/engine/install/"
          target="_blank"
          class="btn btn-primary"
        >
          Install Docker
        </a>
        <a
          href="https://podman.io/getting-started/installation"
          target="_blank"
          class="btn btn-secondary"
        >
          Install Podman
        </a>
      </div>
    </div>

    <!-- Containers List -->
    <template v-else>
      <div
        v-if="dockerStore.containers.length === 0"
        class="empty-state"
      >
        <div class="empty-icon">C</div>
        <h3>No services running</h3>
        <p>Add your first service to get started.</p>
        <button
          class="btn btn-primary"
          @click="showAddModal = true"
        >
          + Add Service
        </button>
      </div>

      <div
        v-else
        class="containers-grid"
      >
        <ContainerCard
          v-for="container in dockerStore.containers"
          :key="container.id"
          :container="container"
          @start="dockerStore.startContainer(container.id)"
          @stop="dockerStore.stopContainer(container.id)"
          @restart="dockerStore.restartContainer(container.id)"
          @remove="dockerStore.removeContainer(container.id)"
        />
      </div>
    </template>

    <!-- Add Service Modal -->
    <Teleport to="body">
      <div
        v-if="showAddModal"
        class="modal-overlay"
        @click.self="closeModal"
      >
        <div class="modal">
          <div class="modal-header">
            <h2>Add Service</h2>
            <button
              class="close-btn"
              @click="closeModal"
            >
              &times;
            </button>
          </div>

          <div class="modal-body">
            <template v-if="!selectedService">
              <!-- Service Selection -->
              <div
                v-for="(templates, category) in dockerStore.templatesByCategory"
                :key="category"
                class="service-category"
              >
                <h3>{{ categoryNames[category] || category }}</h3>
                <div class="service-grid">
                  <button
                    v-for="template in templates"
                    :key="template.id"
                    class="service-option"
                    @click="selectService(template)"
                  >
                    <div class="service-icon">{{ template.name.charAt(0) }}</div>
                    <div class="service-details">
                      <div class="service-name">{{ template.name }}</div>
                      <div class="service-desc">{{ template.description }}</div>
                    </div>
                  </button>
                </div>
              </div>
            </template>

            <template v-else>
              <!-- Service Configuration -->
              <button
                class="back-btn"
                @click="selectedService = null"
              >
                &larr; Back
              </button>

              <div class="config-section">
                <h3>{{ selectedService.name }}</h3>
                <p class="config-desc">{{ selectedService.description }}</p>

                <div class="config-field">
                  <label>Version</label>
                  <select v-model="selectedTag">
                    <option
                      v-for="tag in selectedService.availableTags"
                      :key="tag"
                      :value="tag"
                    >
                      {{ tag }}
                    </option>
                  </select>
                </div>

                <div class="config-field">
                  <label>Ports</label>
                  <div class="ports-preview">
                    <div
                      v-for="port in selectedService.ports"
                      :key="port.container"
                      class="port-item"
                    >
                      {{ port.host }} &rarr; {{ port.container }}/{{ port.protocol }}
                    </div>
                  </div>
                </div>

                <div
                  v-if="Object.keys(selectedService.environment).length > 0"
                  class="config-field"
                >
                  <label>Environment</label>
                  <div class="env-preview">
                    <div
                      v-for="(value, key) in selectedService.environment"
                      :key="key"
                      class="env-item"
                    >
                      <span class="env-key">{{ key }}</span>
                      <span class="env-value">{{ value }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </template>
          </div>

          <div
            v-if="selectedService"
            class="modal-footer"
          >
            <button
              class="btn btn-secondary"
              @click="closeModal"
            >
              Cancel
            </button>
            <button
              class="btn btn-primary"
              :disabled="dockerStore.loading"
              @click="createService"
            >
              {{ dockerStore.loading ? 'Creating...' : 'Create Service' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.containers-page {
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
  font-family: var(--font-mono);
  font-size: 13px;
}

.subtitle .warning {
  color: var(--color-warning);
}

.btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
  text-decoration: none;
  display: inline-block;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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

/* No Runtime */
.no-runtime {
  text-align: center;
  padding: 60px 20px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-warning);
  border-radius: 12px;
}

.no-runtime-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 16px;
  background: rgba(245, 158, 11, 0.1);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-warning);
}

.no-runtime h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 8px;
}

.no-runtime p {
  color: var(--color-text-muted);
  margin: 0 0 20px;
}

.install-options {
  display: flex;
  gap: 12px;
  justify-content: center;
}

/* Empty State */
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

/* Containers Grid */
.containers-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 16px;
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--color-bg-secondary);
  border-radius: 16px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--color-text-muted);
  cursor: pointer;
}

.close-btn:hover {
  color: var(--color-text-primary);
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

/* Service Selection */
.service-category {
  margin-bottom: 24px;
}

.service-category h3 {
  font-size: 12px;
  text-transform: uppercase;
  color: var(--color-text-muted);
  margin-bottom: 12px;
}

.service-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.service-option {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 10px;
  cursor: pointer;
  text-align: left;
  transition: all 0.15s ease;
}

.service-option:hover {
  border-color: var(--color-primary);
  background: var(--color-primary-light);
}

.service-icon {
  width: 40px;
  height: 40px;
  background: var(--color-bg-tertiary);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-primary);
}

.service-name {
  font-weight: 600;
  color: var(--color-text-primary);
}

.service-desc {
  font-size: 12px;
  color: var(--color-text-muted);
}

/* Service Configuration */
.back-btn {
  background: none;
  border: none;
  color: var(--color-primary);
  cursor: pointer;
  font-size: 14px;
  margin-bottom: 20px;
  padding: 0;
}

.config-section h3 {
  margin: 0 0 4px;
}

.config-desc {
  color: var(--color-text-muted);
  margin-bottom: 24px;
}

.config-field {
  margin-bottom: 20px;
}

.config-field label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.config-field select {
  width: 100%;
  padding: 10px 14px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  color: var(--color-text-primary);
  font-size: 14px;
}

.ports-preview,
.env-preview {
  background: var(--color-bg-primary);
  border-radius: 8px;
  padding: 12px;
  font-family: var(--font-mono);
  font-size: 13px;
}

.port-item,
.env-item {
  padding: 4px 0;
}

.env-key {
  color: var(--color-primary);
}

.env-value {
  color: var(--color-text-muted);
  margin-left: 8px;
}

.env-value::before {
  content: '=';
  margin-right: 4px;
}
</style>
