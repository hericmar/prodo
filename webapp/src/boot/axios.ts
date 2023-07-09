import { boot } from 'quasar/wrappers'
import axios, { AxiosError, AxiosInstance } from 'axios'
import { AUTH_TOKEN_NAME, useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $axios: AxiosInstance;
  }
}

// Be careful when using SSR for cross-request state pollution
// due to creating a Singleton instance here;
// If any client changes this (global) instance, it might be a
// good idea to move this instance creation inside of the
// "export default () => {}" function below (which runs individually
// for each client)

export interface ExtendedWindow extends Window {
  PRODO_BASE_URL: string
}

export const BASE_URL = import.meta.env.DEV
  ? 'http://localhost:8000'
  : (window as unknown as ExtendedWindow).PRODO_BASE_URL

const api = axios.create({ baseURL: BASE_URL })

export default boot(({ app }) => {
  // for use inside Vue files (Options API) through this.$axios and this.$api

  app.config.globalProperties.$axios = axios
  // ^ ^ ^ this will allow you to use this.$axios (for Vue Options API form)
  //       so you won't necessarily have to import axios in each vue file

  app.config.globalProperties.$api = api
  // ^ ^ ^ this will allow you to use this.$api (for Vue Options API form)
  //       so you can easily perform requests against your app's API

  api.interceptors.request.use(
    config => {
      config.headers.Authorization = `Bearer ${localStorage.getItem(AUTH_TOKEN_NAME)}`
      return config
    },
    error => {
      return Promise.reject(error)
    }
  )

  api.interceptors.response.use(
    (res) => res,
    async (err: AxiosError) => {
      const onLogout = () => {
        const authStore = useAuthStore()
        authStore.logout()
        router.push({ name: 'login' })
      }

      const request = err.config

      const isLoginRequest = request?.url?.endsWith('/token')
      const isRefreshRequest = request?.url?.endsWith('/token/refresh')

      // ensure the request is not login request
      if (!isLoginRequest && err.response?.status === 401) {
        const authStore = useAuthStore()

        if (!isRefreshRequest) {
          if (!authStore.refreshToken) {
            return onLogout()
          }

          try {
            await authStore.refresh()
            return api(request)
          } catch (e) {
            onLogout()
          }
        } else {
          onLogout()
        }
      }

      return Promise.reject(err)
    }
  )
})

export { api }
