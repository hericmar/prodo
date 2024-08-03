<template>
  <div class=" q-px-lg q-pt-md q-pb-lg">
    <q-btn
      class="text-body1 text-bold"
      flat
      rounded
      no-caps
      label="Add list"
      color="blue-7"
      @click="taskStore.addList({ name: 'New List' })"
    >
    </q-btn>
  </div>
</template>

<script setup lang="ts">
import { useTaskStore } from 'stores/task-store'

const taskStore = useTaskStore()
</script>
