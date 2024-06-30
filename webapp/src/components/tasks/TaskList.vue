<template>
  <q-card
    class="task-list q-pb-md"
    flat
  >
    <q-card-section class="flex justify-between q-pb-none">
      <div class="input-wrapper flex justify-between full-width">
        <!--
        <EditableText
          :readonly="props.virtual"
          :label="props.label"
          @update:modelValue="(newValue: string) => taskStore.updateList(props.uid, { name: newValue })"
        />
        -->
        <q-input
          class="text-h6 text-weight-bold q-mr-sm"
          color="blue-6"
          v-model="name"
          :readonly="props.virtual"
          borderless
          debounce="5000"
          @update:modelValue="onNameUpdate"
        />
        <q-btn-dropdown
          class="q-pa-none"
          style="height: 16px; width: 32px;"
          size="md"
          flat
          rounded
        >
          <q-list>
            <q-item clickable v-close-popup @click="showCompleted = !showCompleted">
              <q-item-section avatar>
                <q-avatar :icon="showCompleted ? 'remove_done' : 'done_all'" color="primary" text-color="white" />
              </q-item-section>
              <q-item-section>
                <q-item-label>
                  {{ showCompleted ? 'Hide completed' : 'Show completed' }}
                </q-item-label>
              </q-item-section>
            </q-item>
            <q-separator />
            <q-item clickable v-close-popup @click="confirmDelete = true">
              <q-item-section avatar>
                <q-avatar icon="delete" color="red" text-color="white" />
              </q-item-section>
              <q-item-section>
                <q-item-label>Delete list</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-btn-dropdown>
      </div>
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
      v-if="!props.virtual"
      class="task-input q-ml-md q-mr-sm q-pl-sm"
      color="blue-6"
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

    <div
      v-if="props.virtual"
      class="q-pt-md"
    >
      <SingleTask
        v-for="task in tasks"
        :key="task.uid"
        :task="task"
        no-drag
      />
    </div>
    <div v-else>
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
    </div>
  </q-card>

  <q-dialog
    v-model="confirmDelete"
    no-backdrop-dismiss
    :on-escape-key="() => confirmDelete = false"
  >
    <!-- @keydown.enter="onDelete" -->
    <q-card>
      <q-card-section class="row items-center">
        <q-avatar icon="remove" color="primary" text-color="white" />
        <span class="q-ml-sm">Do you want to delete this list?</span>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" @click="confirmDelete = false" v-close-popup />
        <q-btn flat label="Delete" color="red" @click="taskStore.removeList(props.uid); confirmDelete = false" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { FilterTaskFn, Task, useTaskStore } from 'stores/task-store'
import { computed, onMounted, ref } from 'vue'
import SingleTask from 'components/tasks/SingleTask.vue'
import emitter from 'src/plugins/mitt'
import draggable from 'vuedraggable'
import EditableText from 'components/toolkit/EditableText.vue'
// import { scroll } from 'quasar'

interface Props {
  uid: string,
  label: string
  virtual?: boolean
  filter?: FilterTaskFn
  onCreated?: (task: Task) => void
}

const props = withDefaults(defineProps<Props>(), {
  filter: (uid: string, tasks: Task[]) => tasks,
  onCreated: () => {
    // do nothing
  }
})

const taskStore = useTaskStore()

const name = ref<string>(props.label)
const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(props.uid, { summary: summary.value }).then((task) => {
    props.onCreated(task)
  })

  summary.value = ''
}

const showCompleted = ref<boolean>(false)

const confirmDelete = ref<boolean>(false)

const tasks = computed({
  get: () => {
    // return props.filter(props.uid, taskStore.tasks).filter(task => showCompleted.value || !task.completed)
    const t = taskStore.tasks
    return props.filter(props.uid, t).filter(task => showCompleted.value || !task.completed)
    /*
    const tabName = tab.value
    return props.filter(tabName, taskStore.tasks)
     */
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

const onNameUpdate = (newValue: string) => {
  taskStore.updateList(props.uid, { name: newValue })
}

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

<style lang="scss">
.task-list {
  overflow: hidden;
  @media (min-width: $breakpoint-xs) {
    width: 400px !important;
    min-width: 400px !important;
  }
}

// .task-input label {
.task-input {
  border-bottom: 1px solid $prodo-grey !important;
  padding-left: 0 !important;
  margin-right: 16px;
  margin-bottom: -1px;
}

.body--dark .task-input {
  border-bottom: 1px solid $separator-dark-color !important;
}

.dnd-ghost {
  visibility: hidden;
}

.dnd-chosen {
  @media (max-width: $breakpoint-xs) {
    background: $yellow-1;
  }
}

.dnd-drag {
  /* opacity: 1 !important; */
}

.input-wrapper {

}
</style>
