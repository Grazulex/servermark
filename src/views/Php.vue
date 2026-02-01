<script setup lang="ts">
import { usePhpStore } from '@/stores'

const phpStore = usePhpStore()
</script>

<template>
  <div class="php-page">
    <header class="page-header">
      <div>
        <h1>PHP Versions</h1>
        <p class="subtitle">Install and manage PHP versions</p>
      </div>
    </header>

    <div class="php-grid">
      <div
        v-for="php in phpStore.versions"
        :key="php.version"
        class="php-card"
        :class="{ active: php.active, installed: php.installed }"
      >
        <div class="php-header">
          <div class="php-version">PHP {{ php.version }}</div>
          <div
            v-if="php.active"
            class="php-badge active"
          >
            Active
          </div>
          <div
            v-else-if="php.installed"
            class="php-badge installed"
          >
            Installed
          </div>
        </div>

        <div
          v-if="php.installed"
          class="php-path"
        >
          {{ php.path }}
        </div>

        <div class="php-actions">
          <template v-if="!php.installed">
            <button
              class="btn btn-primary"
              @click="phpStore.installVersion(php.version)"
            >
              Install
            </button>
          </template>
          <template v-else>
            <button
              v-if="!php.active"
              class="btn btn-success"
              @click="phpStore.switchVersion(php.version)"
            >
              Set Active
            </button>
            <button
              v-if="!php.active"
              class="btn btn-danger"
              @click="phpStore.uninstallVersion(php.version)"
            >
              Uninstall
            </button>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.php-page {
  max-width: 1200px;
}

.page-header {
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

.php-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.php-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 20px;
}

.php-card.active {
  border-color: var(--color-success);
  background: linear-gradient(135deg, var(--color-bg-secondary), rgba(34, 197, 94, 0.05));
}

.php-card.installed {
  border-color: var(--color-primary);
}

.php-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.php-version {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: var(--font-mono);
}

.php-badge {
  font-size: 11px;
  font-weight: 600;
  padding: 4px 10px;
  border-radius: 100px;
  text-transform: uppercase;
}

.php-badge.active {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.php-badge.installed {
  background: rgba(59, 130, 246, 0.1);
  color: var(--color-primary);
}

.php-path {
  font-size: 12px;
  color: var(--color-text-muted);
  font-family: var(--font-mono);
  margin-bottom: 16px;
  padding: 8px 12px;
  background: var(--color-bg-tertiary);
  border-radius: 6px;
}

.php-actions {
  display: flex;
  gap: 8px;
}

.btn {
  flex: 1;
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-success {
  background: var(--color-success);
  color: white;
}

.btn-success:hover {
  background: var(--color-success-hover);
}

.btn-danger {
  background: var(--color-danger);
  color: white;
}

.btn-danger:hover {
  background: var(--color-danger-hover);
}
</style>
