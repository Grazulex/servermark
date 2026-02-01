<script setup lang="ts">
// ServerMark - Main Application Component
import Sidebar from '@/components/Sidebar.vue'
import { useTrayEvents } from '@/composables/useTrayEvents'

// Setup tray event listeners
useTrayEvents()
</script>

<template>
  <div id="app-root">
    <Sidebar />
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style>
#app-root {
  display: flex;
  min-height: 100vh;
  background: var(--color-bg-primary);
}

.main-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}
</style>
