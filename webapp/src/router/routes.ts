import { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from 'stores/auth-store'
import { Platform } from 'quasar'

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
        path: '/login',
        name: 'login',
        component: () => import('pages/LoginPage.vue')
      }
    ]
  },
  {
    path: '/',
    component: () => Platform.is.desktop ? import('layouts/MainLayout.vue') : import('layouts/MobileLayout.vue'),
    beforeEnter: (to, from, next) => {
      const authStore = useAuthStore()
      if (to.meta.requiresAuth && !authStore.isAuthenticated) {
        next({ name: 'login' })
      }
      next()
    },
    children: [
      {
        path: '',
        name: 'index',
        components: {
          default: () => Platform.is.desktop ? import('pages/IndexPage.vue') : import('pages/mobile/index/IndexPage.vue'),
          toolbar: () => import('pages/mobile/index/IndexToolbar.vue')
        },
        meta: {
          requiresAuth: true
        }
      },
      {
        // This is mobile only!
        path: 'list/:uid',
        name: 'list',
        components: {
          default: () => import('pages/mobile/list/ListPage.vue'),
          header: () => import('pages/mobile/list/ListHeader.vue')
          // toolbar: () => import('pages/mobile/list/ListToolbar.vue')
        },
        meta: {
          requiresAuth: true
        }
      },
      {
        // This is mobile only!
        path: 'task/:uid',
        name: 'task',
        components: {
          default: () => import('pages/mobile/task/TaskPage.vue'),
          header: () => import('pages/mobile/task/TaskHeader.vue'),
          toolbar: () => import('pages/mobile/task/TaskToolbar.vue')
        },
        meta: {
          requiresAuth: true
        }
      },
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
