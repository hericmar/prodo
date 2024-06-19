<template>
  <q-item
    class="task q-pa-none q-my-sm"
    :class="{
      'task-greyed': props.task.greyedOut,
      'task-urgency-none': props.task.urgency === Urgency.None,
      'task-urgency-low': props.task.urgency === Urgency.Low,
      'task-urgency-medium': props.task.urgency === Urgency.Medium,
      'task-urgency-high': props.task.urgency === Urgency.High,
    }"
    active-class="task-active"
    clickable
    @click="onTaskClick"
  >
    <q-item-section
      class="task-checkbox q-pr-none"
      side
    >
      <q-checkbox
        v-model="completed"
        @click="onCompletedClick"
        :color="props.task.greyedOut ? 'grey' : 'primary'"
      />
    </q-item-section>

    <q-item-section
      class="q-pl-md q-py-sm"
      style="user-select: none"
    >
      <q-item-label
        :class="{'q-pt-sm': props.task.description}"
      >
        {{ props.task.summary }}
      </q-item-label>
      <q-item-label v-if="props.task.description" caption>
        <div class="task-description" v-html="marked(props.task.description)"></div>
      </q-item-label>
      <q-item-label v-if="props.task.due">
        <div class="flex self-center">
          <q-icon class="q-pr-sm" name="upcoming"></q-icon> {{ formatDateLocal(props.task.due) }}
          <q-tooltip :delay="500">due</q-tooltip>
        </div>
      </q-item-label>
      <q-item-label v-if="props.task.start && props.task.end">
        <div class="flex self-center">
          <q-icon class="q-pr-sm" name="access_time"></q-icon> {{ formatDateLocal(props.task.start, { hideYear: props.task.start.getFullYear() === props.task.end.getFullYear() }) }} - {{ formatDateLocal(props.task.end, { hideYear: props.task.start.getFullYear() === props.task.end.getFullYear() }) }}
          <q-tooltip :delay="500">duration</q-tooltip>
        </div>
      </q-item-label>
      <q-item-label v-if="props.task.rrule" class="q-pt-sm text-grey-7">
        {{ RRule.fromString(props.task.rrule).toText() }}
      </q-item-label>
    </q-item-section>

    <q-item-section side>
      <q-icon
        name="more_vert"
        class="drag-handle text-grey-8 q-gutter-xs"
        size="sm"
      />
    </q-item-section>
  </q-item>
</template>

<script lang="ts" setup>
import { Task, Urgency, useTaskStore } from 'stores/task-store'
import { ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { formatDateLocal } from 'src/utils/datetime'
import { RRule } from 'rrule'
import { marked } from 'marked'

interface Props {
  task: Task
}

const props = defineProps<Props>()

const completed = ref(props.task.completed !== null)

const onEdit = () => {
  emitter.emit('on-edit', { task: props.task })
}

const onTaskClick = () => {
  onEdit()
}

const taskStore = useTaskStore()

const onCompletedClick = () => {
  taskStore.toggle(props.task)
}
</script>

<style lang="scss">
.task {
  min-width: 400px;
  @media (max-width: $breakpoint-xs) {
    min-width: 300px;
  }
  max-width: 100%;
}

.task .q-focus-helper {
  display: none;
}

.missed-due {
  background-color: #C10015;
}

.task-description a {
  word-break: break-word;
}

.task-checkbox {
  border-radius: 12px !important;
}

.task-urgency-none .task-checkbox {
  background-color: inherit;
}

.task-urgency-low .task-checkbox {
  background-color: #FFD600;
}

.task-urgency-medium .task-checkbox {
  background-color: #FF6D00;
}

.task-urgency-high .task-checkbox {
  background-color: #C10015;
}

.task-greyed .task-checkbox {
  background-color: #E0E0E0 !important;
}

.task-active {
  background-color: #a82323 !important;
}

.drag-handle {
  cursor: move;
}

.body--dark .task {
  background: $dark;
}

.task {
  margin-left: 16px;
  margin-right: 16px;
  padding-top: 8px;
  border-top: 1px solid $separator-color;
}

.body--dark {
  * > .task {
    border-color: $separator-dark-color;
  }
}

</style>
