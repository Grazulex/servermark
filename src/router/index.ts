import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/views/Dashboard.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: Dashboard,
    },
    {
      path: '/services',
      name: 'services',
      component: () => import('@/views/Services.vue'),
    },
    {
      path: '/sites',
      name: 'sites',
      component: () => import('@/views/Sites.vue'),
    },
    {
      path: '/php',
      name: 'php',
      component: () => import('@/views/Php.vue'),
    },
    {
      path: '/containers',
      name: 'containers',
      component: () => import('@/views/Containers.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('@/views/Settings.vue'),
    },
    {
      path: '/databases',
      name: 'databases',
      component: () => import('@/views/Databases.vue'),
    },
  ],
})

export default router
