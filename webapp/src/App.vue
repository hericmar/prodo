<template>
  <router-view />
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
import { ref, watch } from 'vue'
import { useTaskStore } from 'stores/task-store'

const lastUpdate = new Date()
let lastVisibleState = true

const $q = useQuasar()
watch(() => $q.appVisible, val => {
  // if (val && lastUpdate.getDate() !== new Date().getDate()) {
  if (val) {
    // app is visible and it's a new day
    const taskStore = useTaskStore()
    taskStore.reload()
  }
  // if app is visible, the value is true
  lastVisibleState = val
})
</script>
