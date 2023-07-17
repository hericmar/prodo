<template>
  <q-card class="task-list q-ma-sm q-ma-lg-lg q-pa-sm">
    <div class="flex justify-between items-baseline">
      <div class="flex">
        <h2 class="q-pl-xs q-mb-xs text-h6">Tasks</h2>
        <q-btn icon="link" flat color="grey" dense rounded @click="onLinkClick"></q-btn>
      </div>
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

    <q-list
      class="q-pt-sm"
    >
      <draggable
        tag="q-list"
        :list="tasks"
        item-key="uid"
        :move="checkMove"
        @start="onDragStart"
        @end="onDragEnd"
      >
        <template #item="{ element }">
          <SingleTask
            :task="element"
          />
        </template>
      </draggable>
    </q-list>
  </q-card>
</template>
<script lang="ts" setup>
import { useTaskStore } from 'stores/task-store'
import { computed, onMounted, ref } from 'vue'
import SingleTask from 'components/SingleTask.vue'
import emitter from 'src/plugins/mitt'
import draggable from 'vuedraggable'

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
    // active
    return taskStore.tasks.filter((task) => {
      return !task.completed || task.rrule
    })
  }
})

const onLinkClick = () => {
  emitter.emit('on-link', {
    list: ''
  })
}

// drag and drop
const dragging = ref<boolean>(false)

const onDragStart = (e: any) => {
  dragging.value = true
  console.log('start', e)
}

const onDragEnd = (e: any) => {
  dragging.value = false
  taskStore.setOrder(e.oldIndex, e.newIndex)
}

const checkMove = (e: any) => {
  console.log(e)
}

// lifecycle
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
