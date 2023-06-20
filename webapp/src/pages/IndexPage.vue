<template>
  <q-page
    class="row items-center justify-evenly"
  >
    <TaskList />
  </q-page>

  <q-dialog v-model="confirm" persistent>
    <q-card>
      <q-card-section class="row items-center">
        <q-avatar icon="remove" color="primary" text-color="white" />
        <span class="q-ml-sm">Do you want to delete this task?</span>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" @click="confirm = false" v-close-popup />
        <q-btn flat label="Delete" color="red" @click="onDelete" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTaskStore } from 'stores/task-store'
import TaskList from 'components/TaskList.vue'
import Task from 'components/SingleTask.vue'
import emitter from 'src/plugins/mitt'

const taskStore = useTaskStore()

// Delete confirmation dialog

const confirm = ref(false)
let taskToDelete = null

emitter.on('on-delete', (e) => {
  taskToDelete = e.task
  confirm.value = true
})

const onDelete = () => {
  taskStore.remove(taskToDelete)
  confirm.value = false
}
</script>
