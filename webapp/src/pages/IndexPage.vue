<template>
  <q-page
    class="row items-center justify-evenly"
  >
    <TaskList />
  </q-page>

  <q-dialog v-model="confirmDelete" persistent>
    <q-card>
      <q-card-section class="row items-center">
        <q-avatar icon="remove" color="primary" text-color="white" />
        <span class="q-ml-sm">Do you want to delete this task?</span>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" @click="confirmDelete = false" v-close-popup />
        <q-btn flat label="Delete" color="red" @click="onDelete" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <q-dialog v-model="confirmEdit" no-backdrop-dismiss :on-escape-key="() => confirmEdit = false">
    <q-card>
      <q-card-section>
        <TaskDetailForm :edited-task="task" />
      </q-card-section>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import TaskList from 'components/TaskList.vue'
import emitter from 'src/plugins/mitt'
import TaskDetailForm from 'components/TaskDetailForm.vue'

const taskStore = useTaskStore()

let task: Task

// Delete confirmation dialog
const confirmDelete = ref(false)

emitter.on('on-delete', (e) => {
  task = e.task
  confirmDelete.value = true
})

const onDelete = () => {
  taskStore.remove(task)
  confirmDelete.value = false
  task = undefined
}

// Edit task dialog
const confirmEdit = ref(false)

emitter.on('on-edit', (e) => {
  task = e.task
  confirmEdit.value = true
})

emitter.on('on-edit-close', () => {
  confirmEdit.value = false
  task = undefined
})
</script>
