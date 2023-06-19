import { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from 'stores/auth-store'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    beforeEnter: (to, from, next) => {
      const authStore = useAuthStore()
      if (to.meta.requiresAuth && !authStore.isAuthenticated) {
        next({ name: 'landing' })
      }
      next()
    },
    children: [
      {
        path: '',
        name: 'index',
        component: () => import('pages/IndexPage.vue'),
        meta: {
          requiresAuth: true
        }
      },
      {
        path: '',
        name: 'landing',
        component: () => import('pages/LandingPage.vue'),
        meta: {
          requiresAuth: false
        }
      },
      {
        path: 'login',
        name: 'login',
        component: () => import('pages/LoginPage.vue'),
        meta: {
          requiresAuth: false
        }
      }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue')
  }
]

export default routes
