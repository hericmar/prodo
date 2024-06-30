<template>
  <q-card
    class="task-card"
    style="overflow-y: hidden"
    flat
  >
    <q-scroll-area style="height: 100%; max-width: 100%;">
      <q-card-section>
        <q-form
          class="flex column q-pt-sm q-pr-md q-gutter-md"
          @keydown.enter="onEnterDown"
        >
          <div class="flex content-center no-wrap">
            <q-input
              class="full-width"
              v-model="task.summary"
              :label="$t('summary')"
              stack-label
              counter
              maxlength="60"
              outlined
            />
            <q-btn
              class="q-mb-md q-ml-sm"
              flat
              rounded
              color="red"
              icon="delete"
              @click="emitter.emit('on-delete', { task: props.editedTask })"
            />
          </div>

          <q-input v-model="task.description" :label="$t('description')" stack-label type="textarea" outlined />

          <q-select
            v-model="listUid"
            :label="$t('list')"
            stack-label
            outlined
            :options="listOptions"
            @update:modelValue="onListUpdate"
          />

          <div>Due date</div>
          <DatetimePicker v-model="task.due" :label="$t('dueDate')"></DatetimePicker>

          <h2 class="text-h6">{{ $t('duration') }}</h2>
          <div>{{ $t('from') }}</div>
          <DatetimePicker
            v-model="task.dtstart"
            label="Start"
            :date-only="wholeDay"
            @update:modelValue="onStartChange"
          />

          <div>{{ $t('to') }}</div>
          <DatetimePicker
            v-model="task.dtend"
            label="Due"
            :date-only="wholeDay"
          />
          <!--
          <div>
            <q-checkbox v-model="wholeDay" @click="onWholeDayClick">{{ $t('task_wholeDay') }}</q-checkbox>
          </div>
          -->

          <h2 class="text-h6">{{ $t('priority') }}</h2>
          <ButtonToggle
            v-model="task.priority"
            :options="[
              { label: $t('none'), value: 0 },
              { label: $t('low'), value: 9 },
              { label: $t('medium'), value: 5 },
              { label: $t('high'), value: 1 }
          ]"
          />

          <h2 class="text-h6">{{ $t('recurrence') }}</h2>
          <RRulePicker
            v-model="task.rrule"
            :dtstart="task.dtstart"
          />
          <q-toolbar v-if="!$q.platform.is.mobile">
            <div class="flex justify-between full-width q-px-xl q-py-md">
              <q-btn
                flat
                rounded
                no-caps
                label="Cancel"
                color="blue-7"
                @click="onClose"
              />
              <q-btn
                flat
                rounded
                no-caps
                label="Save"
                color="blue-7"
                @click="onSave"
              />
            </div>
          </q-toolbar>
        </q-form>
      </q-card-section>
    </q-scroll-area>
    <q-toolbar
      v-if="$q.platform.is.mobile"
      class="task-toolbar"
    >
      <div class="flex justify-between full-width q-px-xl q-py-md">
        <q-btn
          flat
          rounded
          no-caps
          label="Cancel"
          color="blue-7"
          @click="onClose"
        />
        <q-btn
          flat
          rounded
          no-caps
          label="Save"
          color="blue-7"
          @click="onSave"
        />
      </div>
    </q-toolbar>
  </q-card>
</template>

<script lang="ts" setup>
import { Task, useTaskStore } from 'stores/task-store'
import { PropType, ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { isTimeSet } from 'src/utils/datetime'
import DatetimePicker from 'components/toolkit/DatetimePicker.vue'
import RRulePicker from 'components/toolkit/RRulePicker.vue'
import ButtonToggle from 'components/toolkit/ButtonToggle.vue'
import api from 'src/api'

const props = defineProps({
  editedTask: {
    type: Object as PropType<Task>,
    required: true
  }
})

const taskStore = useTaskStore()

const task = ref<Task>(props.editedTask)
// TODO: only single list is supported
const listOptions = taskStore.lists.filter(list => !list.isVirtual).map(list => ({ label: list.name, value: list.uid }))
const list = listOptions.find(l => l.value === task.value.lists.values().next().value)
const listUid = ref<{label: string, value: string} | null>(list || null)

// handle from and to fields
const wholeDay = ref<boolean>(task.value.dtstart ? isTimeSet(task.value.dtstart) : false)

const onStartChange = (value: Date) => {
  // onWholeDayClick()
}

const onListUpdate = (option: { label: string, value: string }) => {
  const oldLists = task.value.lists.values()
  for (const oldListUid of oldLists) {
    api.task.move(oldListUid, option.value, task.value.uid)
  }
  task.value.lists = new Set([option.value])
}

/*
const onWholeDayClick = () => {
  if (wholeDay.value && task.value.start) {
    task.value.end = task.value.start
  }
}
 */

const onEnterDown = (event: KeyboardEvent) => {
  if (event.ctrlKey) {
    onSave()
  }
}

const onClose = () => {
  emitter.emit('on-edit-close', {})
}

const onSave = () => {
  taskStore.update(task.value)
    .then(() => {
      onClose()
    })
}
</script>

<style lang="scss">
.task-card {
  width: 500px;
  height: 100%;

  @media (max-width: $breakpoint-xs) {
    width: 100%;
  }
}

.task-toolbar {
  position: sticky;
  bottom: 0;
  margin: 0;
  padding-bottom: 0;
  margin-bottom: -16px;
  width: 100vw;
  background-color: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(10px);
}

.body--dark {
  .task-toolbar {
    background-color: rgba(0, 0, 0, 0.7);
  }
}

@media (max-width: $breakpoint-xs) {
  .task-toolbar {
    border-top: 1px solid $separator-color;
  }

  .body--dark {
    .task-toolbar {
      border-top: 1px solid $separator-dark-color;
    }
  }
}

</style>
