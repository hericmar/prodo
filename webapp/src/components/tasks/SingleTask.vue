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
      <q-item-label
        v-if="props.task.description"
        caption
        class="q-mb-none"
      >
        <div class="marked-text" v-html="marked(props.task.description)"></div>
      </q-item-label>
      <q-item-label v-if="props.task.due">
        <div class="flex self-center">
          <q-icon class="q-pr-sm" name="upcoming"></q-icon> {{ formatDateLocal(props.task.due) }}
          <q-tooltip :delay="500">due</q-tooltip>
        </div>
      </q-item-label>
      <q-item-label v-if="props.task.dtstart && props.task.dtend">
        <div class="flex self-center">
          <q-icon class="q-pr-sm" name="access_time"></q-icon> {{ formatDateLocal(props.task.dtstart, { hideYear: props.task.dtstart.getFullYear() === props.task.dtend.getFullYear() }) }} - {{ formatDateLocal(props.task.dtend, { hideYear: props.task.dtstart.getFullYear() === props.task.dtend.getFullYear() }) }}
          <q-tooltip :delay="500">duration</q-tooltip>
        </div>
      </q-item-label>
      <q-item-label v-if="props.task.rrule" class="q-pt-sm text-grey-7">
        {{ RRule.fromString(props.task.rrule).toText() }}
      </q-item-label>
    </q-item-section>

    <q-item-section
      v-if="!props.noDrag"
      side
    >
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
  noDrag?: boolean
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
  margin: 0 16px -1px 16px;
  // padding-top: 8px;
  /* border-top: 1px solid $separator-color; */
}

* > .task {
  border-top: 1px solid $prodo-grey;
  border-bottom: 1px solid $prodo-grey;
  padding-top: 8px;
  padding-bottom: 8px;
}

.body--dark {
  * > .task {
    border-color: $separator-dark-color;
  }
}

.marked-text {
  p:last-of-type {
    margin-bottom: 0;
  }
}
</style>
