<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSitesStore } from '@/stores/sites'
import { usePhpStore } from '@/stores/php'
import { open } from '@tauri-apps/plugin-dialog'
import { open as openUrl } from '@tauri-apps/plugin-shell'
import SiteCard from '@/components/SiteCard.vue'
import type { FrameworkTemplate } from '@/stores/sites'

const sitesStore = useSitesStore()
const phpStore = usePhpStore()

// Modal state
const showAddModal = ref(false)
const addMode = ref<'create' | 'import' | 'clone'>('create')

// Form state
const projectName = ref('')
const projectPath = ref('')
const selectedFramework = ref<FrameworkTemplate | null>(null)
const selectedVersion = ref('')
const selectedPhpVersion = ref('')
const importPath = ref('')
const cloneUrl = ref('')

onMounted(async () => {
  await Promise.all([
    sitesStore.fetchSites(),
    sitesStore.fetchFrameworks(),
    sitesStore.fetchConfig(),
    phpStore.fetchVersions(),
  ])
})

function openAddModal() {
  showAddModal.value = true
  addMode.value = 'create'
  resetForm()
}

function closeModal() {
  showAddModal.value = false
  resetForm()
}

function resetForm() {
  projectName.value = ''
  projectPath.value = sitesStore.config.sites_path || ''
  selectedFramework.value = null
  selectedVersion.value = ''
  selectedPhpVersion.value = phpStore.activeVersion?.version || ''
  importPath.value = ''
  cloneUrl.value = ''
}

function selectFramework(framework: FrameworkTemplate) {
  selectedFramework.value = framework
  selectedVersion.value = framework.default_version
}

async function handleCreate() {
  if (!selectedFramework.value || !projectName.value) return

  try {
    await sitesStore.createProject(
      projectName.value,
      selectedFramework.value.id,
      selectedVersion.value,
      selectedPhpVersion.value || undefined,
      projectPath.value || undefined
    )
    closeModal()
  } catch (e) {
    console.error('Failed to create project:', e)
  }
}

async function browseFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Project Directory',
  })
  if (selected && typeof selected === 'string') {
    importPath.value = selected
  }
}

async function browseProjectPath() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Location for New Project',
    defaultPath: projectPath.value || undefined,
  })
  if (selected && typeof selected === 'string') {
    projectPath.value = selected
  }
}

async function handleImport() {
  if (!importPath.value) return

  try {
    await sitesStore.addSite(
      importPath.value,
      projectName.value || undefined,
      selectedPhpVersion.value || undefined
    )
    closeModal()
  } catch (e) {
    console.error('Failed to import site:', e)
  }
}

async function handleClone() {
  if (!cloneUrl.value) return

  try {
    await sitesStore.cloneRepository(
      cloneUrl.value,
      projectName.value || undefined,
      selectedPhpVersion.value || undefined
    )
    closeModal()
  } catch (e) {
    console.error('Failed to clone repository:', e)
  }
}

async function openSite(domain: string, secured: boolean) {
  const url = `http${secured ? 's' : ''}://${domain}`
  await openUrl(url)
}
</script>

<template>
  <div class="sites-page">
    <header class="page-header">
      <div>
        <h1>Sites</h1>
        <p class="subtitle">
          {{ sitesStore.sites.length }} site{{ sitesStore.sites.length !== 1 ? 's' : '' }} configured
          <span v-if="sitesStore.config.tld">&middot; .{{ sitesStore.config.tld }}</span>
        </p>
      </div>
      <button class="btn btn-primary" @click="openAddModal">
        + Add Site
      </button>
    </header>

    <div v-if="sitesStore.error" class="error-banner">
      {{ sitesStore.error }}
    </div>

    <div v-if="sitesStore.sites.length === 0" class="empty-state">
      <div class="empty-icon">S</div>
      <h3>No sites configured</h3>
      <p>Add your first site to get started with local development.</p>
      <button class="btn btn-primary" @click="openAddModal">
        + Add Site
      </button>
    </div>

    <div v-else class="sites-grid">
      <SiteCard
        v-for="site in sitesStore.sites"
        :key="site.id"
        :site="site"
        @open="openSite(site.domain, site.secured)"
        @secure="sitesStore.secureSite(site.id)"
        @unsecure="sitesStore.unsecureSite(site.id)"
        @settings="() => {}"
        @upgrade-laravel="() => {}"
        @remove="sitesStore.removeSite(site.id)"
      />
    </div>

    <!-- Add Site Modal -->
    <Teleport to="body">
      <div v-if="showAddModal" class="modal-overlay" @click.self="closeModal">
        <div class="modal add-site-modal">
          <div class="modal-header">
            <h2>Add Site</h2>
            <button class="close-btn" @click="closeModal">&times;</button>
          </div>

          <!-- Mode Tabs -->
          <div class="mode-tabs">
            <button
              class="mode-tab"
              :class="{ active: addMode === 'create' }"
              @click="addMode = 'create'"
            >
              Create New
            </button>
            <button
              class="mode-tab"
              :class="{ active: addMode === 'import' }"
              @click="addMode = 'import'"
            >
              Import Local
            </button>
            <button
              class="mode-tab"
              :class="{ active: addMode === 'clone' }"
              @click="addMode = 'clone'"
            >
              Clone Repo
            </button>
          </div>

          <div class="modal-body">
            <!-- Create New Project -->
            <template v-if="addMode === 'create'">
              <div v-if="!selectedFramework" class="framework-grid">
                <button
                  v-for="framework in sitesStore.frameworks"
                  :key="framework.id"
                  class="framework-option"
                  @click="selectFramework(framework)"
                >
                  <div class="framework-icon">{{ framework.name.charAt(0) }}</div>
                  <div class="framework-info">
                    <div class="framework-name">{{ framework.name }}</div>
                    <div class="framework-desc">{{ framework.description }}</div>
                  </div>
                </button>
              </div>

              <template v-else>
                <button class="back-btn" @click="selectedFramework = null">
                  &larr; Back
                </button>

                <div class="form-section">
                  <h3>{{ selectedFramework.name }} Project</h3>

                  <div class="form-field">
                    <label>Project Name</label>
                    <input
                      v-model="projectName"
                      type="text"
                      placeholder="my-awesome-project"
                    />
                  </div>

                  <div class="form-field" v-if="selectedFramework.versions.length > 1">
                    <label>Version</label>
                    <select v-model="selectedVersion">
                      <option
                        v-for="ver in selectedFramework.versions"
                        :key="ver"
                        :value="ver"
                      >
                        {{ ver }}
                      </option>
                    </select>
                  </div>

                  <div class="form-field">
                    <label>PHP Version</label>
                    <select v-model="selectedPhpVersion">
                      <option value="">Default ({{ phpStore.activeVersion?.version || 'system' }})</option>
                      <option
                        v-for="php in phpStore.installedVersions"
                        :key="php.version"
                        :value="php.version"
                      >
                        PHP {{ php.version }}
                      </option>
                    </select>
                  </div>

                  <div class="form-field">
                    <label>Location</label>
                    <div class="input-with-button">
                      <input
                        v-model="projectPath"
                        type="text"
                        placeholder="/home/user/Sites"
                      />
                      <button class="btn btn-secondary" @click="browseProjectPath">
                        Browse
                      </button>
                    </div>
                    <small>Project will be created at: {{ projectPath }}/{{ projectName || 'project-name' }}</small>
                  </div>

                  <div class="form-info">
                    <div class="info-item">
                      <span class="info-label">Domain:</span>
                      <span class="info-value">{{ (projectName || 'project-name').toLowerCase().replace(/\s+/g, '-') }}.{{ sitesStore.config.tld }}</span>
                    </div>
                  </div>
                </div>
              </template>
            </template>

            <!-- Import Local Project -->
            <template v-if="addMode === 'import'">
              <div class="form-section">
                <div class="form-field">
                  <label>Project Path</label>
                  <div class="input-with-button">
                    <input
                      v-model="importPath"
                      type="text"
                      placeholder="/home/user/Code/my-project"
                      readonly
                    />
                    <button class="btn btn-secondary" @click="browseFolder">
                      Browse...
                    </button>
                  </div>
                  <small>Click "Browse" to select your project directory</small>
                </div>

                <div class="form-field">
                  <label>Site Name (optional)</label>
                  <input
                    v-model="projectName"
                    type="text"
                    placeholder="Leave empty to use directory name"
                  />
                </div>

                <div class="form-field">
                  <label>PHP Version</label>
                  <select v-model="selectedPhpVersion">
                    <option value="">Default ({{ phpStore.activeVersion?.version || 'system' }})</option>
                    <option
                      v-for="php in phpStore.installedVersions"
                      :key="php.version"
                      :value="php.version"
                    >
                      PHP {{ php.version }}
                    </option>
                  </select>
                </div>
              </div>
            </template>

            <!-- Clone Repository -->
            <template v-if="addMode === 'clone'">
              <div class="form-section">
                <div class="form-field">
                  <label>Repository URL</label>
                  <input
                    v-model="cloneUrl"
                    type="text"
                    placeholder="https://github.com/user/repo.git"
                  />
                </div>

                <div class="form-field">
                  <label>Project Name (optional)</label>
                  <input
                    v-model="projectName"
                    type="text"
                    placeholder="Leave empty to use repo name"
                  />
                </div>

                <div class="form-field">
                  <label>PHP Version</label>
                  <select v-model="selectedPhpVersion">
                    <option value="">Default ({{ phpStore.activeVersion?.version || 'system' }})</option>
                    <option
                      v-for="php in phpStore.installedVersions"
                      :key="php.version"
                      :value="php.version"
                    >
                      PHP {{ php.version }}
                    </option>
                  </select>
                </div>

                <div class="form-info">
                  <p class="info-note">
                    After cloning, ServerMark will automatically:
                  </p>
                  <ul>
                    <li>Run <code>composer install</code> if composer.json exists</li>
                    <li>Run <code>npm install</code> if package.json exists</li>
                    <li>Copy .env.example to .env (Laravel)</li>
                    <li>Generate application key (Laravel)</li>
                  </ul>
                </div>
              </div>
            </template>
          </div>

          <div class="modal-footer">
            <button class="btn btn-secondary" @click="closeModal">Cancel</button>
            <button
              v-if="addMode === 'create'"
              class="btn btn-primary"
              :disabled="!selectedFramework || !projectName || sitesStore.loading"
              @click="handleCreate"
            >
              {{ sitesStore.loading ? 'Creating...' : 'Create Project' }}
            </button>
            <button
              v-if="addMode === 'import'"
              class="btn btn-primary"
              :disabled="!importPath || sitesStore.loading"
              @click="handleImport"
            >
              {{ sitesStore.loading ? 'Importing...' : 'Import Site' }}
            </button>
            <button
              v-if="addMode === 'clone'"
              class="btn btn-primary"
              :disabled="!cloneUrl || sitesStore.loading"
              @click="handleClone"
            >
              {{ sitesStore.loading ? 'Cloning...' : 'Clone & Setup' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.sites-page {
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
  font-size: 14px;
}

.error-banner {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid var(--color-danger);
  color: var(--color-danger);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 24px;
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

.sites-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
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
  max-height: 85vh;
  display: flex;
  flex-direction: column;
}

.add-site-modal {
  max-width: 700px;
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

.mode-tabs {
  display: flex;
  border-bottom: 1px solid var(--color-border);
}

.mode-tab {
  flex: 1;
  padding: 14px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--color-text-muted);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.mode-tab:hover {
  color: var(--color-text-primary);
}

.mode-tab.active {
  color: var(--color-primary);
  border-bottom-color: var(--color-primary);
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

/* Framework Grid */
.framework-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.framework-option {
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

.framework-option:hover {
  border-color: var(--color-primary);
  background: var(--color-primary-light);
}

.framework-icon {
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

.framework-name {
  font-weight: 600;
  color: var(--color-text-primary);
}

.framework-desc {
  font-size: 12px;
  color: var(--color-text-muted);
}

/* Form */
.back-btn {
  background: none;
  border: none;
  color: var(--color-primary);
  cursor: pointer;
  font-size: 14px;
  margin-bottom: 20px;
  padding: 0;
}

.form-section h3 {
  margin: 0 0 20px;
}

.form-field {
  margin-bottom: 20px;
}

.form-field label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.form-field input,
.form-field select {
  width: 100%;
  padding: 10px 14px;
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  color: var(--color-text-primary);
  font-size: 14px;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
}

.form-field select {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%239ca3af' d='M2 4l4 4 4-4'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 36px;
  cursor: pointer;
}

.form-field input:focus,
.form-field select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.form-field small {
  display: block;
  margin-top: 6px;
  font-size: 12px;
  color: var(--color-text-muted);
}

.form-field select option {
  background: #1e1e2e;
  color: #cdd6f4;
  padding: 8px 12px;
}

.input-with-button {
  display: flex;
  gap: 8px;
}

.input-with-button input {
  flex: 1;
  cursor: pointer;
}

.input-with-button .btn {
  flex-shrink: 0;
  padding: 10px 16px;
}

.form-info {
  background: var(--color-bg-tertiary);
  border-radius: 8px;
  padding: 16px;
  font-size: 13px;
}

.info-item {
  display: flex;
  gap: 8px;
  padding: 4px 0;
}

.info-label {
  color: var(--color-text-muted);
}

.info-value {
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}

.info-note {
  margin: 0 0 12px;
  color: var(--color-text-secondary);
}

.form-info ul {
  margin: 0;
  padding-left: 20px;
  color: var(--color-text-muted);
}

.form-info li {
  padding: 4px 0;
}

.form-info code {
  background: var(--color-bg-primary);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: var(--font-mono);
  font-size: 12px;
}
</style>
