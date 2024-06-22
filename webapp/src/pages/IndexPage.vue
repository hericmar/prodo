<template>
  <!--
  <q-pull-to-refresh :disable="dragging" @refresh="onRefresh" no-mouse style="height: 100vh" scroll-target="body">
  </q-pull-to-refresh>
  -->

  <q-page
    class="list-container flex no-wrap q-gutter-lg q-pt-lg"
    v-touch-pan.capture.down="onSwipeDown"
    @touchend="onMouseUp"
  >
    <TaskList
      :label="$t('dailyTasks')"
      :filter="filterDailyTasks"
      :onCreated="onDailyTaskCreated"
    />
    <TaskList
      :label="$t('tasks')"
      :filter="filterTasks"
    />
    <div v-if="!$q.platform.is.mobile">
      <q-btn
        icon="add"
        flat
        round
      />
    </div>
  </q-page>

  <q-dialog
    v-model="confirmDelete"
    no-backdrop-dismiss
    :on-escape-key="() => confirmDelete = false"
  >
    <!-- @keydown.enter="onDelete" -->
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

  <q-dialog
    v-model="confirmEdit"
    no-backdrop-dismiss
    :maximized="$q.platform.is.mobile"
    :on-escape-key="() => confirmEdit = false"
  >
    <TaskDetailForm :edited-task="task" />
  </q-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import api from 'src/api'
import TaskList from 'components/tasks/TaskList.vue'
import emitter from 'src/plugins/mitt'
import TaskDetailForm from 'components/tasks/TaskDetail.vue'

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
  emitter.emit('on-edit-close', {})
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

const dragging = ref(false)
emitter.on('on-drag-start', () => {
  dragging.value = true
})
emitter.on('on-drag-end', () => {
  dragging.value = false
})

const reloading = ref(false)
const onSwipeDown = (e) => {
  const threshold = 100

  if (dragging.value) {
    return
  }

  if (window.scrollY < 0) {
    if (e.distance.y > threshold && !reloading.value) {
      reloading.value = true
    }
  }
}

//

const filterDailyTasks = (tasks: Task[]) => {
  // inactive
  // task.greyedOut
  return tasks.filter((task) => {
    return task.rrule
  })
}

const onDailyTaskCreated = (task: Task) => {
  task.rrule = 'FREQ=DAILY'
  api.task.update(task.uid, task)
}

const filterTasks = (tasks: Task[]) => {
  return tasks.filter((task) => {
    return !task.rrule
  })
}

//

const onMouseUp = () => {
  if (!reloading.value) {
    return
  }

  taskStore.reload().then(() => {
    console.log('reloaded')
    reloading.value = false
  })
}

/*
const onRefresh = (done: any) => {
  taskStore.reload()
  done()
}
 */

// lifecycle
onMounted(() => {
  taskStore.init()
})
</script>

<style lang="scss">
.list-container {
  align-items: flex-start;

  @media (max-width: $breakpoint-xs) {
    align-items: normal;
    flex-direction: column;
  }
}

.list-container > * {
  min-height: calc(100vh - $toolbar-min-height - 48px);

  @media (min-width: $breakpoint-xs) {
    margin-left: 48px;
    min-height: 500px !important;
  }
}
</style>
