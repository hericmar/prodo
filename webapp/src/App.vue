<template>
  <router-view />
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
import { watch } from 'vue'
import { useTaskStore } from 'stores/task-store'

let lastUpdate = new Date()

const $q = useQuasar()
watch(() => $q.appVisible, val => {
  if (val && lastUpdate.getDate() !== new Date().getDate()) {
    // app is visible and it's a new day
    useTaskStore().reload()
    lastUpdate = new Date()
  }
})
</script>
