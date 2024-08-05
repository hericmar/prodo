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
          :readonly="props.list.isVirtual"
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
            <q-item
              clickable
              v-close-popup
              @click="confirmArchive"
            >
              <q-item-section avatar>
                <q-avatar icon="archive" color="blue-6" text-color="white" />
              </q-item-section>
              <q-item-section>
                <q-item-label>Archive list</q-item-label>
              </q-item-section>
            </q-item>
            <q-item
              v-if="!props.list.isVirtual"
              clickable
              v-close-popup
              @click="confirmDelete"
            >
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
      v-if="!props.list.isVirtual"
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
      v-if="props.list.isVirtual"
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
</template>

<script lang="ts" setup>
import { TaskList, Task, useTaskStore } from 'stores/task-store'
import { computed, ref } from 'vue'
import SingleTask from 'components/tasks/SingleTask.vue'
import emitter from 'src/plugins/mitt'
import draggable from 'vuedraggable'
import { useQuasar } from 'quasar'
import { useRouter } from 'vue-router'
// import { scroll } from 'quasar'

interface Props {
  list: TaskList,
}

const props = defineProps<Props>()

const taskStore = useTaskStore()

const name = ref<string>(props.list.name)
const summary = ref<string>('')
const onAddTask = () => {
  if (!summary.value) {
    return
  }

  taskStore.addTask(props.list.uid, { summary: summary.value })

  summary.value = ''
}

const showCompleted = ref<boolean>(false)

const tasks = computed({
  get: () => {
    const filter = props.list.onFilter
    return filter(props.list).filter(task => showCompleted.value || !task.completed)
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

  taskStore.setOrder(props.list.uid, droppedTask, e.newIndex)
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
  taskStore.updateList(props.list.uid, { name: newValue })
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

// Dialogs

const $q = useQuasar()
const router = useRouter()

const confirmArchive = () => {
  $q.dialog({
    title: 'Do you want to archive this list?',
    message: 'This will hide the list from the main view. You can unarchive it from the settings in the profile page.',
    cancel: true,
    persistent: true
  }).onOk(() => {
    taskStore.updateList(props.list.uid, { is_archived: true })
    if ($q.platform.is.mobile) {
      router.back()
    }
  })
}

const confirmDelete = () => {
  $q.dialog({
    title: 'Confirm',
    message: 'Do you want to delete this list?',
    cancel: true,
    persistent: true
  }).onOk(() => {
    taskStore.removeList(props.list.uid)
    if ($q.platform.is.mobile) {
      router.back()
    }
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
