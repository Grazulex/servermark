<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { usePhpStore, type PhpExtension } from '@/stores/php'
import { useConfigStore } from '@/stores/config'

const props = defineProps<{
  version: string
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  installed: []
}>()

const phpStore = usePhpStore()
const configStore = useConfigStore()

const selectedExtensions = ref<Set<string>>(new Set())

const groupedExtensions = computed(() => {
  const groups: Record<string, PhpExtension[]> = {
    required: [],
    recommended: [],
    optional: [],
  }

  for (const ext of phpStore.extensions) {
    if (groups[ext.category]) {
      groups[ext.category].push(ext)
    }
  }

  return groups
})

onMounted(async () => {
  await phpStore.fetchExtensions()
  // Pre-select default extensions
  for (const ext of phpStore.extensions) {
    if (ext.default_selected) {
      selectedExtensions.value.add(ext.name)
    }
  }
})

function toggleExtension(name: string, category: string) {
  // Required extensions cannot be deselected
  if (category === 'required') return

  if (selectedExtensions.value.has(name)) {
    selectedExtensions.value.delete(name)
  } else {
    selectedExtensions.value.add(name)
  }
}

function selectAll(category: string) {
  for (const ext of groupedExtensions.value[category] || []) {
    selectedExtensions.value.add(ext.name)
  }
}

function deselectAll(category: string) {
  if (category === 'required') return
  for (const ext of groupedExtensions.value[category] || []) {
    selectedExtensions.value.delete(ext.name)
  }
}

async function handleInstall() {
  try {
    await phpStore.installWithExtensions(
      props.version,
      Array.from(selectedExtensions.value),
      configStore.config.packageManager
    )
    emit('installed')
    emit('close')
  } catch (e) {
    console.error('Install failed:', e)
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="modal-overlay"
      @click.self="emit('close')"
    >
      <div class="modal">
        <header class="modal-header">
          <h2>Install PHP {{ version }}</h2>
          <button
            class="close-btn"
            @click="emit('close')"
          >
            &times;
          </button>
        </header>

        <div class="modal-body">
          <p class="modal-description">
            Select the extensions to install with PHP {{ version }}.
            Required extensions are automatically selected.
          </p>

          <!-- PPA Warning -->
          <div
            v-if="!phpStore.isPpaInstalled"
            class="ppa-warning"
          >
            <div class="warning-icon">!</div>
            <div class="warning-content">
              <strong>PPA ondrej/php not detected</strong>
              <p>
                For the best PHP support on Ubuntu/Debian, we recommend adding the ondrej/php PPA.
                This provides the latest PHP versions and extensions.
              </p>
              <!-- PPA Progress -->
              <div
                v-if="phpStore.addingPpa && phpStore.ppaProgress"
                class="ppa-progress"
              >
                <div class="progress-step">{{ phpStore.ppaProgress.step }}</div>
                <div class="progress-bar-container small">
                  <div
                    class="progress-bar"
                    :style="{ width: `${(phpStore.ppaProgress.current_step / phpStore.ppaProgress.total_steps) * 100}%` }"
                  ></div>
                </div>
                <div class="progress-count">
                  Step {{ phpStore.ppaProgress.current_step }} of {{ phpStore.ppaProgress.total_steps }}
                </div>
              </div>
              <button
                v-else
                class="btn btn-warning"
                :disabled="phpStore.addingPpa"
                @click="phpStore.addPpa()"
              >
                {{ phpStore.addingPpa ? 'Adding PPA...' : 'Add PPA' }}
              </button>
            </div>
          </div>

          <!-- Extensions List -->
          <div class="extensions-container">
            <!-- Required -->
            <div class="extension-group">
              <div class="group-header">
                <h3>Required</h3>
                <span class="group-badge required">Always installed</span>
              </div>
              <div class="extensions-grid">
                <label
                  v-for="ext in groupedExtensions.required"
                  :key="ext.name"
                  class="extension-item required"
                >
                  <input
                    type="checkbox"
                    :checked="true"
                    disabled
                  >
                  <div class="extension-info">
                    <span class="extension-name">{{ ext.display_name }}</span>
                    <span class="extension-desc">{{ ext.description }}</span>
                  </div>
                </label>
              </div>
            </div>

            <!-- Recommended -->
            <div class="extension-group">
              <div class="group-header">
                <h3>Recommended</h3>
                <div class="group-actions">
                  <button
                    class="link-btn"
                    @click="selectAll('recommended')"
                  >
                    Select all
                  </button>
                  <span class="divider">|</span>
                  <button
                    class="link-btn"
                    @click="deselectAll('recommended')"
                  >
                    Deselect all
                  </button>
                </div>
              </div>
              <div class="extensions-grid">
                <label
                  v-for="ext in groupedExtensions.recommended"
                  :key="ext.name"
                  class="extension-item"
                >
                  <input
                    type="checkbox"
                    :checked="selectedExtensions.has(ext.name)"
                    @change="toggleExtension(ext.name, 'recommended')"
                  >
                  <div class="extension-info">
                    <span class="extension-name">{{ ext.display_name }}</span>
                    <span class="extension-desc">{{ ext.description }}</span>
                  </div>
                </label>
              </div>
            </div>

            <!-- Optional -->
            <div class="extension-group">
              <div class="group-header">
                <h3>Optional</h3>
                <div class="group-actions">
                  <button
                    class="link-btn"
                    @click="selectAll('optional')"
                  >
                    Select all
                  </button>
                  <span class="divider">|</span>
                  <button
                    class="link-btn"
                    @click="deselectAll('optional')"
                  >
                    Deselect all
                  </button>
                </div>
              </div>
              <div class="extensions-grid">
                <label
                  v-for="ext in groupedExtensions.optional"
                  :key="ext.name"
                  class="extension-item"
                >
                  <input
                    type="checkbox"
                    :checked="selectedExtensions.has(ext.name)"
                    @change="toggleExtension(ext.name, 'optional')"
                  >
                  <div class="extension-info">
                    <span class="extension-name">{{ ext.display_name }}</span>
                    <span class="extension-desc">{{ ext.description }}</span>
                  </div>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Install Progress Overlay -->
        <div
          v-if="phpStore.installing"
          class="progress-overlay"
        >
          <div class="progress-content">
            <div class="progress-spinner"></div>
            <div class="progress-text">Installing PHP {{ version }}...</div>
            <div class="progress-subtext">This may take a few minutes</div>
          </div>
        </div>

        <footer class="modal-footer">
          <div class="selected-count">
            {{ selectedExtensions.size }} extensions selected
          </div>
          <div class="modal-actions">
            <button
              class="btn btn-secondary"
              :disabled="phpStore.installing"
              @click="emit('close')"
            >
              Cancel
            </button>
            <button
              class="btn btn-primary"
              :disabled="phpStore.installing"
              @click="handleInstall"
            >
              {{ phpStore.installing ? 'Installing...' : `Install PHP ${version}` }}
            </button>
          </div>
        </footer>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal {
  background: var(--color-bg-primary);
  border-radius: 16px;
  width: 90%;
  max-width: 640px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  border: 1px solid var(--color-border);
  position: relative;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  font-size: 28px;
  color: var(--color-text-muted);
  cursor: pointer;
  line-height: 1;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.modal-description {
  color: var(--color-text-secondary);
  margin: 0 0 20px;
  font-size: 14px;
}

.ppa-warning {
  display: flex;
  gap: 16px;
  padding: 16px;
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.3);
  border-radius: 12px;
  margin-bottom: 24px;
}

.warning-icon {
  width: 32px;
  height: 32px;
  background: rgba(245, 158, 11, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  color: #f59e0b;
  flex-shrink: 0;
}

.warning-content {
  flex: 1;
}

.warning-content strong {
  color: #f59e0b;
  display: block;
  margin-bottom: 4px;
}

.warning-content p {
  color: var(--color-text-secondary);
  font-size: 13px;
  margin: 0 0 12px;
}

.btn-warning {
  background: #f59e0b;
  color: #000;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-warning:hover:not(:disabled) {
  background: #d97706;
}

.btn-warning:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.extensions-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.extension-group {
  background: var(--color-bg-secondary);
  border-radius: 12px;
  padding: 16px;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.group-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.group-badge {
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 100px;
  background: var(--color-bg-tertiary);
  color: var(--color-text-muted);
}

.group-badge.required {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.group-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.link-btn {
  background: none;
  border: none;
  color: var(--color-primary);
  font-size: 12px;
  cursor: pointer;
  padding: 0;
}

.link-btn:hover {
  text-decoration: underline;
}

.divider {
  color: var(--color-border);
  font-size: 12px;
}

.extensions-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.extension-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  background: var(--color-bg-tertiary);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.extension-item:hover {
  background: var(--color-bg-hover);
}

.extension-item.required {
  opacity: 0.7;
  cursor: default;
}

.extension-item input[type="checkbox"] {
  width: 16px;
  height: 16px;
  margin-top: 2px;
  accent-color: var(--color-primary);
  flex-shrink: 0;
}

.extension-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.extension-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.extension-desc {
  font-size: 11px;
  color: var(--color-text-muted);
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-top: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
  border-radius: 0 0 16px 16px;
}

.selected-count {
  font-size: 13px;
  color: var(--color-text-muted);
}

.modal-actions {
  display: flex;
  gap: 12px;
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

.btn:disabled {
  opacity: 0.6;
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

/* Progress Overlay */
.progress-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 16px;
  z-index: 10;
}

.progress-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 40px;
}

.progress-spinner {
  width: 56px;
  height: 56px;
  border: 4px solid var(--color-bg-tertiary);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.progress-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.progress-subtext {
  font-size: 14px;
  color: var(--color-text-muted);
}

.progress-bar-container.small {
  height: 6px;
  margin: 8px 0;
  width: 200px;
  background: rgba(245, 158, 11, 0.2);
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: var(--color-primary);
  border-radius: 4px;
  transition: width 0.3s ease;
}

/* PPA Progress */
.ppa-progress {
  margin-top: 8px;
}

.ppa-progress .progress-step {
  font-size: 13px;
  color: var(--color-text-secondary);
  text-align: left;
}

.ppa-progress .progress-count {
  text-align: left;
}
</style>
