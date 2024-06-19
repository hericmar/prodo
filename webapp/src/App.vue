<template>
  <router-view />
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
import { watch } from 'vue'
import { useTaskStore } from 'stores/task-store'
import { useSettingsStore } from 'stores/settings-store'

let lastUpdate = new Date()

useSettingsStore().init()

const $q = useQuasar()
watch(() => $q.appVisible, val => {
  if (val && lastUpdate.getDate() !== new Date().getDate()) {
    // app is visible and it's a new day
    useTaskStore().reload()
    lastUpdate = new Date()
  }
})

// dark and light mode
const dark = localStorage.getItem('dark')
if (dark === null) {
  $q.dark.set('auto')
} else {
  $q.dark.set(dark === 'true')
}
</script>
