import { defineStore } from 'pinia'

const SETTINGS_STORAGE_KEY = 'settings'
export type Settings = {
  preferDarkMode?: boolean,
  showLandingPage: boolean,
  preferredLocale?: string
}

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    showLandingPage: true,
    preferredLocale: undefined,
    preferDarkMode: undefined
  } as Settings),
  actions: {
    init () {
      const settings = localStorage.getItem(SETTINGS_STORAGE_KEY)
      if (settings) {
        this.$patch(JSON.parse(settings))
      }
    },
    updateSettings (settings: Partial<Settings>) {
      this.$patch(settings)
      localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(this.$state))
    }
  }
})
