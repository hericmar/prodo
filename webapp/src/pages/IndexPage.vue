<template>
  <q-page
    class="row justify-evenly"
  >
    <TaskList />
  </q-page>

  <q-dialog
    v-model="confirmDelete"
    no-backdrop-dismiss
    :on-escape-key="() => confirmDelete = false"
    @keydown.enter="onDelete"
  >
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
    <q-card class="task-card">
      <q-card-section>
        <TaskDetailForm :edited-task="task" />
      </q-card-section>
    </q-card>
  </q-dialog>

  <q-dialog v-model="confirmLink" no-backdrop-dismiss :on-escape-key="() => confirmLink = false">
    <q-card class="task-card">
      <q-card-section>
        <h2 class="text-h6">{{ $t('subscription_header') }}</h2>
        <p>{{ $t('subscription_url_description') }}</p>
      </q-card-section>
      <q-card-section>
        <span v-if="link === ''" class="flex items-center">
          <q-btn icon="add" round flat @click="onCreateLink"></q-btn>
          {{ $t('subscription_generate') }}
        </span>
        <q-input v-else outlined v-model="link" :label="$t('subscription_url')" readonly>
          <template v-slot:after>
            <q-btn flat icon="content_copy" @click="onLinkCopy" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-section>
        <q-btn flat label="Remove" color="red" @click="onRemoveLink" v-if="link !== ''" />
      </q-card-section>
      <q-card-actions>
        <q-btn flat label="Close" color="primary" @click="confirmLink = false" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import TaskList from 'components/TaskList.vue'
import emitter from 'src/plugins/mitt'
import TaskDetailForm from 'components/TaskDetailForm.vue'
import api from 'src/api'
import { BASE_URL } from 'src/boot/axios'
import { copyToClipboard } from 'quasar'

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

// Create link dialog

const link = ref('')

api.ical.get().then((res) => {
  link.value = BASE_URL + '/api/v1/ical/' + res.data.secret
})

const confirmLink = ref(false)
emitter.on('on-link', () => {
  confirmLink.value = true
})

const onCreateLink = () => {
  api.ical.create().then((res) => {
    api.ical.get().then((res) => {
      link.value = BASE_URL + '/api/v1/ical/' + res.data.secret
    })
  })
}

const onRemoveLink = () => {
  api.ical.delete().then(() => {
    link.value = ''
  })
}

const onLinkCopy = () => {
  copyToClipboard(link.value)
    .then(() => {
      // success!
    })
    .catch(() => {
      // fail
    })
}

const onLinkClose = () => {
  confirmLink.value = false
}
</script>

<style lang="sass">
.task-card
  @media (max-width: $breakpoint-xs)
    width: 100%
  @media (min-width: $breakpoint-xs)
    width: 400px
</style>
