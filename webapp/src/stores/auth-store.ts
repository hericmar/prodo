import { defineStore } from 'pinia'

const REFRESH_TOKEN_NAME = 'refresh'
const AUTH_TOKEN_NAME = 'token'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    counter: 0,
    token: localStorage.getItem(AUTH_TOKEN_NAME),
    refreshToken: localStorage.getItem(REFRESH_TOKEN_NAME)
  }),
  getters: {
    doubleCount: (state) => state.counter * 2,
    isAuthenticated: (state) => !!state.token
  },
  actions: {
    async login (username: string, password: string) {
      localStorage.setItem(REFRESH_TOKEN_NAME, 'refresh')
      localStorage.setItem(AUTH_TOKEN_NAME, 'token')
      this.token = 'token'
      this.refreshToken = 'refresh'

      return Promise.resolve()
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
