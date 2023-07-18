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

    <draggable
      tag="div"
      class="q-list q-pt-sm"
      v-model="tasks"
      item-key="uid"
      @start="onDragStart"
      @change="onDragChange"
      @end="onDragEnd"
      delay="150"
      :delayOnTouchOnly="true"
      touchStartThreshold: 5
      ghostClass="dnd-ghost"
      chosenClass="dnd-chosen"
      direction="vertical"
      :scrollFn="onScroll"
      animation="200"
      scrollSpeed=10
      scrollSensitivity="200"
    >
      <template #item="{ element }">
        <SingleTask
          :task="element"
        />
      </template>
    </draggable>
    {{ message }}
  </q-card>
</template>
<script lang="ts" setup>
import { useTaskStore } from 'stores/task-store'
import { computed, onMounted, ref } from 'vue'
import SingleTask from 'components/SingleTask.vue'
import emitter from 'src/plugins/mitt'
import draggable from 'vuedraggable'
// import { scroll } from 'quasar'

const taskStore = useTaskStore()

const tab = ref<string>('active')
// const tab = ref<string>('all')

const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(summary.value)

  summary.value = ''
}

const tasks = computed({
  get: () => {
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
  },
  set: () => {
    console.log('drag end')
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
  emitter.emit('on-drag-start')
}

const onDragChange = (e) => {
  console.log('onDragChange', e)
}

const message = ref('')
const onDragEnd = (e: any) => {
  dragging.value = false
  emitter.emit('on-drag-end')

  if (e.oldIndex === e.newIndex) {
    return
  }

  console.log(e.oldIndex, e.newIndex)

  const droppedTask = tasks.value[e.oldIndex]
  const droppedIndex = taskStore.tasks.indexOf(tasks.value[e.newIndex])

  // message.value = 'oldIndex: ' + oldIndex + ', newIndex: ' + newIndex
  taskStore.setOrder(droppedTask, droppedIndex)

  /*
  const task = tasks.value[e.newIndex]
  // console.log(task.summary, 'to', tasks.value[e.oldIndex].summary)
  const oldIndex = tasks.value[e.newIndex].order

  // index of task before the dropped task in the filtered list,
  // when the dropped task is the first task, it's 0.
  const newIndex = tasks.value[e.oldIndex].order - 1 === -1 ? 0 : tasks.value[e.oldIndex].order - 1

  console.log(oldIndex, 'to', newIndex)

  taskStore.setOrder(oldIndex, newIndex)

   */

  // console.log(droppedTask.summary, 'to', otherTask.summary)

  /*
  const task = tasks.value[e.newIndex]
  const otherTask = tasks.value[e.oldIndex]

  console.log(task.summary, 'to', otherTask.summary)

  const oldIndex = taskStore.tasks.indexOf(otherTask)
  const newIndex = taskStore.tasks.indexOf(task)

  console.log('oldIndex:', oldIndex, 'newIndex:', newIndex)
  console.log(taskStore.tasks[oldIndex].summary, 'to', taskStore.tasks[newIndex].summary)

   */
}

// const onScroll = (offsetX: number, offsetY: number, originalEvent: any, touchEvt: any, hoverTargetEl: any) => {
const onScroll = (offsetX: number, offsetY: number) => {
  // Smooth scrolling is not working properly, so we disable it for now.

  window.scrollBy({
    left: 0,
    top: 0 // offsetY
    // behavior: 'smooth'
  })

  // const { getVerticalScrollPosition, setVerticalScrollPosition } = scroll
  // setVerticalScrollPosition(window, getVerticalScrollPosition(window) + offsetY, 2)
}

// lifecycle
onMounted(() => {
  taskStore.init()
})
</script>

<style lang="sass">
.dnd-ghost
  visibility: hidden

.dnd-chosen
  /* background: blue */

.dnd-drag
  /* background: green */

.task-list
  @media (max-width: $breakpoint-xs)
    width: 100% !important
  @media (min-width: $breakpoint-xs)
    width: 500px !important
</style>
