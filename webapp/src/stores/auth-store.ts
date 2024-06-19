import { defineStore } from 'pinia'
import api from 'src/api'

type User = {
  uid: string
}

type RootState = {
  user: User | null
  message: string
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: localStorage.getItem('user') ? JSON.parse(localStorage.getItem('user') as string) : null,
    message: ''
  } as RootState),
  getters: {
    isAuthenticated: (state) => !!state.user
  },
  actions: {
    async login (username: string, password: string) {
      await api.auth.login(username, password)
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
      await this.getUser()
    },
    logout () {
      this.user = null
      localStorage.removeItem('user')
      return api.auth.logout()
    },
    getUser () {
      return api.auth.user().then(response => {
        this.user = response.data
        localStorage.setItem('user', JSON.stringify(response.data))
      })
    }
  }
})
