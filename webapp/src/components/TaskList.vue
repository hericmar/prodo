<template>
  <div class="task-list q-ma-sm q-ma-lg-lg q-pt-sm">
    <div class="flex justify-between items-end">
      <div class="flex">
        <h2 class="q-pl-xs q-mb-none text-h6">Tasks</h2>
        <q-btn
          icon="link"
          flat
          color="grey"
          dense
          rounded
          @click="onLinkClick" />
      </div>
      <q-tabs
        v-model="tab"
        class="flex"
        align="right"
        shrink
        dense
        style="max-width: 300px"
      >
        <q-tab name="active" label="Active" />
        <q-tab name="completed" label="Completed" />
        <q-tab name="all" label="All" />
      </q-tabs>
    </div>

    <q-card
      class="q-pa-sm q-pa-sm-md bg-grey-3 rounded-borders"
      flat
    >
      <q-input
        outlined
        bg-color="white"
        v-model="summary"
        :label="$t('task_newInput')"
        counter
        maxlength="60"
        @keydown.enter="onAddTask"
      >
        <template v-if="$q.screen.xs" v-slot:append>
          <q-btn
            flat
            round
            icon="add"
            @click="onAddTask">
          </q-btn>
        </template>
      </q-input>

      <draggable
        tag="div"
        class="q-list q-pt-sm"
        v-model="tasks"
        item-key="uid"
        @start="onDragStart"
        @end="onDragEnd"
        delay="150"
        direction="vertical"
        :delayOnTouchOnly="true"
        touchStartThreshold: 5
        ghostClass="dnd-ghost"
        chosenClass="dnd-chosen"
        dragClass="dnd-drag"
        fallback-class="dnd-drag"
        :force-fallback="true"
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
    </q-card>
  </div>
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

const isFocused = ref<boolean>(false)

// drag and drop
const dragging = ref<boolean>(false)

const onDragStart = (e: any) => {
  console.log(e)
  e.clone.classList.remove('dnd-ghost')
  dragging.value = true
  emitter.emit('on-drag-start')
}

const message = ref('')
const onDragEnd = (e: any) => {
  dragging.value = false
  emitter.emit('on-drag-end')

  if (e.oldIndex === e.newIndex) {
    return
  }

  const droppedTask = tasks.value[e.oldIndex]
  const droppedIndex = taskStore.tasks.indexOf(tasks.value[e.newIndex])

  taskStore.setOrder(droppedTask, droppedIndex)
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
  @media (max-width: $breakpoint-xs)
    background: $yellow-1

.dnd-drag
  /* opacity: 1 !important */

.task-list
  @media (max-width: $breakpoint-xs)
    width: 100% !important
  @media (min-width: $breakpoint-xs)
    width: 500px !important
</style>
