import { boot } from 'quasar/wrappers'
import axios, { AxiosError, AxiosInstance } from 'axios'
import { useAuthStore } from 'stores/auth-store'
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

const api = axios.create()

export default boot(({ app }) => {
  // for use inside Vue files (Options API) through this.$axios and this.$api

  app.config.globalProperties.$axios = axios
  // ^ ^ ^ this will allow you to use this.$axios (for Vue Options API form)
  //       so you won't necessarily have to import axios in each vue file

  app.config.globalProperties.$api = api
  // ^ ^ ^ this will allow you to use this.$api (for Vue Options API form)
  //       so you can easily perform requests against your app's API

  /*
  api.interceptors.request.use(
    config => {
      config.headers.Authorization = `Bearer ${localStorage.getItem(AUTH_TOKEN_NAME)}`
      return config
    },
    error => {
      return Promise.reject(error)
    }
  )
   */

  api.interceptors.response.use(
    (res) => res,
    async (err: AxiosError) => {
      const onLogout = () => {
        const authStore = useAuthStore()
        authStore.logout()
        router.push({ name: 'login' })
      }

      const request = err.config

      const isLoginRequest = request?.url?.endsWith('/api/v1/auth/login')
      const isLogoutRequest = request?.url?.endsWith('/api/v1/auth/logout')

      // ensure the request is not login request
      if (!isLoginRequest && !isLogoutRequest && err.response?.status === 401) {
        onLogout()
      }

      return Promise.reject(err)
    }
  )
})

export { api }
