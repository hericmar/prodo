import { defineStore } from 'pinia'
import api from 'src/api'

export const REFRESH_TOKEN_NAME = 'refresh'
export const AUTH_TOKEN_NAME = 'token'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: localStorage.getItem(AUTH_TOKEN_NAME),
    refreshToken: localStorage.getItem(REFRESH_TOKEN_NAME),
    message: ''
  }),
  getters: {
    isAuthenticated: (state) => !!state.token
  },
  actions: {
    async login (username: string, password: string) {
      return api.auth.login(username, password)
        .then((response) => {
          const { refresh, access } = response.data
          this.setToken(access)
          localStorage.setItem(REFRESH_TOKEN_NAME, refresh)
          this.refreshToken = refresh
        })
        .catch((error) => {
          if (error.response.status === 401) {
            this.message = 'login_unauthorized'
            throw new Error('login_unauthorized')
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
    refresh () {
      return api.auth.refresh(this.refreshToken)
        .then((response) => {
          const { access } = response.data
          this.setToken(access)
        })
    },
    setToken (access: string) {
      this.token = access
      localStorage.setItem(AUTH_TOKEN_NAME, access)
    }
  }
})
