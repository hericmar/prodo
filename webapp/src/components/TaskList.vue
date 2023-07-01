<template>
  <q-card class="q-pa-sm task-list">
    <div class="flex justify-between items-baseline">
      <h2 class="q-pl-xs q-mb-xs text-h6">Tasks</h2>
      <q-tabs
        v-model="tab"
        class="q-pb-sm flex"
        align="right"
        shrink
        style="max-width: 300px"
      >
        <q-tab name="active" label="Active" />
        <q-tab name="completed" label="Completed" />
        <q-tab name="all" label="All" />
      </q-tabs>
    </div>
    <q-input
      outlined
      v-model="summary"
      :label="$t('task_newInput')"
      @keydown.enter="onAddTask" />

    <q-list class="q-pt-sm">
      <Task v-for="task in tasks" :key="task.uid" :task="task" />
    </q-list>
  </q-card>
</template>
<script lang="ts" setup>
import { useTaskStore } from 'stores/task-store'
import { computed, onMounted, ref } from 'vue'
import Task from 'components/SingleTask.vue'

const taskStore = useTaskStore()

const tab = ref<string>('active')

const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(summary.value)

  summary.value = ''
}

const tasks = computed(() => {
  if (tab.value === 'all') {
    return taskStore.tasks
  }
  if (tab.value === 'completed') {
    return taskStore.tasks.filter((task) => {
      return task.completed
    })
  } else {
    return taskStore.tasks.filter((task) => {
      return !task.completed
    })
  }
})

onMounted(() => {
  taskStore.init()
})
</script>

<style lang="sass">
.task-list
  @media (max-width: $breakpoint-xs)
    width: 100% !important
  @media (min-width: $breakpoint-xs)
    width: 500px !important
</style>
