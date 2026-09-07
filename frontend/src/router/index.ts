import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import DashboardLayout from '../layouts/DashboardLayout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: () => import('../pages/auth/LoginView.vue'),
      meta: { guestOnly: true }
    },
    {
      path: '/',
      component: DashboardLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          redirect: '/dashboard'
        },
        {
          path: 'dashboard',
          name: 'dashboard',
          component: () => import('../pages/dashboard/Index.vue')
        },
        // Placeholder for other nav items
        {
          path: 'sales',
          name: 'sales',
          component: () => import('../pages/dashboard/Index.vue') // Placeholder
        },
        {
          path: 'reports',
          name: 'reports',
          component: () => import('../pages/dashboard/Index.vue') // Placeholder
        },
        {
          path: 'customers',
          name: 'customers',
          component: () => import('../pages/dashboard/Index.vue') // Placeholder
        },
        {
          path: 'settings',
          name: 'settings',
          component: () => import('../pages/dashboard/Index.vue') // Placeholder
        }
      ]
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../pages/AboutView.vue')
    }
  ]
})

// Navigation Guard
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()

  // Try to recover auth state from token if not explicitly authenticated yet
  if (!authStore.isAuthenticated) {
     authStore.checkAuth()
  }

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login')
  } else if (to.meta.guestOnly && authStore.isAuthenticated) {
    next('/dashboard')
  } else {
    next()
  }
})

export default router
