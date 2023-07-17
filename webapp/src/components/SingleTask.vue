<template>
  <q-item
    class="q-pl-none"
    clickable
    @dblclick="onTaskClick"
  >
    <div class="task-side" :class="{ 'missed-due': (props.task.due && props.task.due < new Date()) }"></div>

    <q-item-section class="q-pr-none" side>
      <q-checkbox
        v-model="completed"
        @click="onCompletedClick"
        :color="props.task.greyedOut ? 'grey' : 'primary'"
      />
    </q-item-section>

    <q-item-section class="q-pl-md" :class="{ greyed: props.task.greyedOut }" style="user-select: none">
      <q-item-label>{{ props.task.summary }}</q-item-label>
      <q-item-label v-if="props.task.description" caption>
        <div class="task-description" v-html="marked(props.task.description)"></div>
      </q-item-label>
      <q-item-label v-if="props.task.due" class="q-pt-sm">
        <div class="flex self-center">
          <q-icon class="q-pr-sm" name="access_time"></q-icon> {{ formatDate(props.task.due) }}
        </div>
      </q-item-label>
      <q-item-label v-if="props.task.rrule" class="q-pt-sm text-grey-7">
        {{ RRule.fromString(props.task.rrule).toText() }}
      </q-item-label>
    </q-item-section>

    <q-item-section side>
      <div class="text-grey-8 q-gutter-xs">
        <q-btn size="12px" flat dense round icon="more_vert">
          <q-menu>
            <q-list>
              <q-item clickable v-close-popup>
                <q-item-section
                  @click="onEdit"
                >Edit</q-item-section>
              </q-item>
              <q-item clickable v-close-popup>
                <q-item-section
                  @click="onDelete"
                >Delete</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
      </div>
    </q-item-section>
  </q-item>
</template>

<script lang="ts" setup>
import { Task, useTaskStore } from 'stores/task-store'
import { ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { formatDate } from 'src/utils/datetime'
import { RRule } from 'rrule'
import { marked } from 'marked'

interface Props {
  task: Task
}

const props = defineProps<Props>()

const completed = ref(props.task.completed !== null)

const onDelete = () => {
  emitter.emit('on-delete', { task: props.task })
}

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

<style>
.missed-due {
  background-color: #C10015;
}

.task-side {
  width: 10px;
  border-radius: 4px;
  position: relative;
}

.greyed {
  filter: brightness(150%);
}

.task-description a {
  word-break: break-word;
}
</style>
