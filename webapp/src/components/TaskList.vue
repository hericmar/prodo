<template>
  <q-input outlined
           v-model="summary"
           :label="$t('task_newInput')"
           @keydown.enter="onAddTask" />

  <q-list>
    <Task v-for="task in tasks" :key="task.uid" :task="task" />
  </q-list>
</template>

<script lang="ts" setup>
import { useTaskStore } from 'stores/task-store'
import { computed, ref } from 'vue'
import Task from 'components/SingleTask.vue'

const taskStore = useTaskStore()

const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(summary.value)

  summary.value = ''
}

const tasks = computed(() => taskStore.tasks)
</script>
