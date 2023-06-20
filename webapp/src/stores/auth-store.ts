import { defineStore } from 'pinia'
import api from 'src/api'

export const REFRESH_TOKEN_NAME = 'refresh'
export const AUTH_TOKEN_NAME = 'token'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    counter: 0,
    token: localStorage.getItem(AUTH_TOKEN_NAME),
    refreshToken: localStorage.getItem(REFRESH_TOKEN_NAME),
    message: ''
  }),
  getters: {
    doubleCount: (state) => state.counter * 2,
    isAuthenticated: (state) => !!state.token
  },
  actions: {
    async login (username: string, password: string) {
      return api.auth.login(username, password)
        .then((response) => {
          const { refresh, access } = response.data
          localStorage.setItem(AUTH_TOKEN_NAME, access)
          localStorage.setItem(REFRESH_TOKEN_NAME, refresh)
          this.token = access
          this.refreshToken = refresh
        })
        .catch((error) => {
          if (error.response.status === 401) {
            this.message = 'login_unauthorized'
          }
        })
    },
    logout () {
      localStorage.removeItem(REFRESH_TOKEN_NAME)
      localStorage.removeItem(AUTH_TOKEN_NAME)
      this.token = ''
      this.refreshToken = ''

      return Promise.resolve()
    },
    refreshToken () {
      this.counter++
    }
  }
})
