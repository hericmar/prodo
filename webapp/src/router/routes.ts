import { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from 'stores/auth-store'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'landing',
    component: () => import('layouts/LandingLayout.vue'),
    beforeEnter: (to, from, next) => {
      if (useAuthStore().isAuthenticated) {
        next({ name: 'index' })
      }
      next()
    },
    children: [
      {
        path: '',
        name: 'landing',
        component: () => import('pages/LandingPage.vue'),
        meta: {
          requiresAuth: false
        }
      },
      {
        path: '/login',
        name: 'login',
        component: () => import('pages/LoginPage.vue')
      }
    ]
  },
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
      /*
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
      },
       */
      {
        path: 'profile',
        name: 'profile',
        component: () => import('pages/ProfilePage.vue'),
        meta: {
          requiresAuth: true
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
