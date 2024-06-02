import { defineStore } from 'pinia'
import api from 'src/api'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    message: ''
  }),
  getters: {
    isAuthenticated: (state) => !!state.user
  },
  actions: {
    async login (username: string, password: string) {
      return api.auth.login(username, password)
        .then(() => {
          this.message = ''
        })
        .catch(error => {
          if (error.response.status === 401) {
            this.message = 'login_unauthorized'
            throw new Error('login_unauthorized')
          } else {
            this.message = 'error'
            throw new Error('error')
          }
        })
    },
    logout () {
      return api.auth.logout()
    },
    user () {
      return api.auth.user()
    }
  }
})
