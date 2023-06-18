import { defineStore } from 'pinia'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    counter: 0
  }),
  getters: {
    doubleCount: (state) => state.counter * 2
  },
  actions: {
    login () {
      this.counter++
    },
    logout () {
    }
  }
})
