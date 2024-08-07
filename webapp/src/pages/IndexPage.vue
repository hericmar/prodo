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
      v-for="list in taskStore.lists.filter(list => !list.is_archived)" :key="list.uid"
      :list="list"
    />
    <div
      v-if="!$q.platform.is.mobile"
      class="q-mr-md"
    >
      <q-btn
        class="q-mr-md"
        icon="add"
        flat
        round
        @click="taskStore.addList({ name: 'New List' })"
      />
    </div>
  </q-page>

  <q-dialog
    v-model="confirmEdit"
    no-backdrop-dismiss
    :maximized="$q.platform.is.mobile"
    :on-escape-key="() => confirmEdit = false"
  >
    <TaskDetailForm :edited-task="task!" />
  </q-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Task, useTaskStore } from 'stores/task-store'
import TaskList from 'components/tasks/TaskList.vue'
import emitter from 'src/plugins/mitt'
import TaskDetailForm from 'components/tasks/TaskDetail.vue'

const taskStore = useTaskStore()

let task: Task | undefined

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
  // min-width: max-content;

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
