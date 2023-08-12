<template>
  <!--
  <q-pull-to-refresh :disable="dragging" @refresh="onRefresh" no-mouse style="height: 100vh" scroll-target="body">
  </q-pull-to-refresh>
  -->

  <q-page
    class="row justify-evenly"
    v-touch-pan.capture.down="onSwipeDown"
    @touchend="onMouseUp"
  >
    <TaskList
      :label="$t('dailyTasks')"
      :startTab="dailyTasksTab"
      :tabs="dailyTasksTabs"
      :filter="filterDailyTasks"
      :onCreated="onDailyTaskCreated"
    />
    <TaskList
      :label="$t('tasks')"
      :startTab="tasksTab"
      :tabs="tasksTabs"
      :filter="filterTasks"
    />
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
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import api from 'src/api'
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

const dailyTasksTab = 'all'
const dailyTasksTabs = ['all', 'active', 'inactive']

const tasksTab = 'active'
const tasksTabs = ['active', 'completed', 'all']

const filterDailyTasks = (tab: string, tasks: Task[]) => {
  const daily = tasks.filter((task) => {
    return task.rrule
  })

  if (tab === 'all') {
    return daily
  }
  if (tab === 'active') {
    return daily.filter((task) => {
      return !task.greyedOut
    })
  } else {
    // inactive
    return daily.filter((task) => {
      return task.greyedOut
    })
  }
}

const onDailyTaskCreated = (task: Task) => {
  task.rrule = 'FREQ=DAILY'
  api.task.update(task.uid, task)
}

const filterTasks = (tab: string, tasks: Task[]) => {
  const notDaily = tasks.filter((task) => {
    return !task.rrule
  })

  if (tab === 'all') {
    return notDaily
  }
  if (tab === 'completed') {
    return notDaily.filter((task) => {
      return task.completed
    })
  } else {
    // active
    return notDaily.filter((task) => {
      return !task.completed || task.rrule
    })
  }
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
</script>

<style lang="sass">
.task-card
  @media (max-width: $breakpoint-xs)
    width: 100%
  @media (min-width: $breakpoint-xs)
    width: 400px
</style>
