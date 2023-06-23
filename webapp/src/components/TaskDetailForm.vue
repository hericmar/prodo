<template>
  <q-form
    class="q-py-sm q-gutter-md flex column"
    @keydown.enter="onSave"
  >
    <q-input v-model="task.summary" :label="$t('summary')" stack-label counter maxlength="40" outlined></q-input>

    <q-input v-model="task.description" :label="$t('description')" stack-label type="textarea" outlined></q-input>

    <div class="block">
      <div class="row items-center">
        <div>
          <q-btn
            v-if="hasInterval"
            class=""
            @click="onIntervalRemove"
            flat
            round
            icon="remove" />
          <q-btn
            v-else
            class=""
            @click="onIntervalAdd"
            flat
            round
            icon="add" />
        </div>
        <h2 class="text-h6">{{ $t('task_interval') }}</h2>
      </div>

      <div v-if="hasInterval" class="block q-col-gutter-md">
        <q-checkbox v-model="wholeDay">{{ $t('task_wholeDay') }}</q-checkbox>
        <div>
          <div class="row q-pb-md">
            <DatetimePicker v-model="task.start" label="Start" :date-only="wholeDay" />
          </div>

          <div class="row">
            <DatetimePicker v-model="task.end" label="Due" :date-only="wholeDay" />
          </div>
        </div>
      </div>
    </div>

    <!-- https://github.com/jakubroztocil/rrule -->
    <!-- TODO rrule input https://codepen.io/theeternalsw0rd/pen/JLMjGx -->

    <div class="q-pa-md q-gutter-sm">
      <q-btn flat label="Cancel" color="primary" @click="onClose" />
      <q-btn flat label="Save" color="primary" @click="onSave" />
    </div>
  </q-form>
</template>

<script lang="ts" setup>
import { Task, useTaskStore } from 'stores/task-store'
import { PropType, ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { isTimeSet } from 'src/utils/datetime'
import DatetimePicker from 'components/toolkit/DatetimePicker.vue'

const props = defineProps({
  editedTask: {
    type: Object as PropType<Task>,
    required: true
  }
})

const taskStore = useTaskStore()

const task = ref<Task>(props.editedTask)

console.log('task', task.value)

const hasInterval = ref(!!task.value.start)
console.log('hasInterval', hasInterval.value)

const onIntervalAdd = () => {
  task.value.start = new Date()
  task.value.end = new Date()
  hasInterval.value = true
}

const onIntervalRemove = () => {
  task.value.start = null
  task.value.end = null
  hasInterval.value = false
}

const wholeDay = ref<boolean>(task.value.start ? isTimeSet(task.value.start) : false)

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
