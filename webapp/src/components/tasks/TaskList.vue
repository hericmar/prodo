<template>
  <q-card flat>
    <q-card-section class="flex justify-between q-pb-none">
      <h2 class="text-h6 q-my-sm">{{ props.label }}</h2>
      <!--
      <q-chip
        v-if="tab !== 'completed'"
        class="q-mt-sm self-start"
        dense
        color="primary"
        text-color="white"
      >
        {{ tasks.reduce((count, task) => task.completed ? count : count + 1, 0) }}
      </q-chip>
      -->
    </q-card-section>
    <q-input
      class="q-ml-md q-mr-sm q-pl-sm"
      v-model="summary"
      :label="$t('task_newInput')"
      borderless
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
    <!--
    <SingleTask
      v-for="task in tasks"
      :key="task.uid"
      :task="task"
    />
    -->
    <draggable
      v-model="tasks"
      item-key="uid"
      handle=".drag-handle"
      @start="onDragStart"
      @end="onDragEnd"
      direction="vertical"
      touchStartThreshold: 5
      ghostClass="dnd-ghost"
      chosenClass="dnd-chosen"
      dragClass="dnd-drag"
      fallbackClass="dnd-drag"
      animation="200"
      scrollSpeed=15
      scrollSensitivity="200"
      :bubbleScroll="false"
      :dragoverBubble="false"
    >
      <template #item="{ element }">
        <SingleTask
          :task="element"
        />
      </template>
    </draggable>
  </q-card>
</template>

<script lang="ts" setup>
import { Task, useTaskStore } from 'stores/task-store'
import { computed, onMounted, ref } from 'vue'
import SingleTask from 'components/tasks/SingleTask.vue'
import emitter from 'src/plugins/mitt'
import draggable from 'vuedraggable'
// import { scroll } from 'quasar'

const props = defineProps({
  label: {
    type: String,
    required: true
  },
  filter: {
    type: Function,
    required: true
  },
  tabs: {
    type: Array,
    required: true
  },
  startTab: {
    type: String,
    required: true
  },
  onCreated: {
    type: Function,
    required: false,
    default: () => {
      // do nothing
    }
  }
})

const taskStore = useTaskStore()

const tab = ref<string>(props.startTab)

const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(summary.value).then((task) => {
    props.onCreated(task)
  })

  summary.value = ''
}

const tasks = computed({
  get: () => {
    const tabName = tab.value
    return props.filter(tabName, taskStore.tasks)
  },
  set: () => {
    console.log('drag end')
  }
})

const similarTasks = computed(() => {
  return findSimilar(summary.value, tasks.value)
})

const isFocused = ref<boolean>(false)

// drag and drop
const dragging = ref<boolean>(false)

const onDragStart = (e: any) => {
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

/*
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
 */

// move outside
const findSimilar = (summary: string, tasks: Array<Task>) => {
  // console.log('find similar')

  if (summary.length < 3) {
    return []
  }

  return tasks.filter((task: Task) => {
    return task.summary.toLowerCase().includes(summary.toLowerCase())
  })
}
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
  overflow: hidden
  @media (max-width: $breakpoint-xs)
    width: 100% !important
  @media (min-width: $breakpoint-xs)
    width: 500px !important
</style>
