import { defineStore } from 'pinia'
import api from 'src/api'

const SETTINGS_STORAGE_KEY = 'settings'
export type Settings = {
  preferDarkMode?: boolean,
  preferredLocale?: string
  timezone: string
  subscriptionSecret: string,
  calendarLastSync?: Date
}

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    preferredLocale: undefined,
    preferDarkMode: undefined,
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
    subscriptionSecret: ''
  } as Settings),
  actions: {
    async init () {
      const settings = localStorage.getItem(SETTINGS_STORAGE_KEY)
      if (settings) {
        this.$patch(JSON.parse(settings))
      }
      await api.ical.get().then(({ data }) => {
        this.timezone = data.timezone
        this.subscriptionSecret = data.secret
        this.calendarLastSync = data.last_synced_at || undefined
      })
        .catch(() => {
          this.timezone = Intl.DateTimeFormat().resolvedOptions().timeZone
          this.subscriptionSecret = ''
        })
      this.$patch(this.$state)
    },
    async generateSubscription () {
      const { data } = await api.ical.create({
        timezone: this.timezone
      })
      this.subscriptionSecret = data.secret
    },
    async deleteSubscription () {
      await api.ical.delete()
      this.subscriptionSecret = ''
    },
    async updateSettings (settings: Partial<Settings>) {
      this.$patch(settings)
      localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(this.$state))
      if (this.subscriptionSecret) {
        await api.ical.update({ timezone: this.timezone })
      }
    }
  }
})
