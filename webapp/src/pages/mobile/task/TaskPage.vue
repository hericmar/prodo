<template>
  <q-page class="">
    <TaskDetail
      v-if="task"
      style="border-radius: 0 !important;"
      :edited-task="task"
    />
  </q-page>
</template>

<script setup lang="ts">
import TaskDetail from 'components/tasks/TaskDetail.vue'
import { ref } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import { useRoute } from 'vue-router'

const task = ref<Task | undefined>(undefined)

const route = useRoute()

const taskStore = useTaskStore()
if (taskStore.tasks.length === 0) {
  taskStore.init().then(() => {
    task.value = taskStore.tasks.find(task => task.uid === route.params.uid)
  })
}

task.value = taskStore.tasks.find(task => task.uid === route.params.uid)
</script>
