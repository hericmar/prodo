<template>
  <q-form
    class="q-py-sm q-gutter-md flex column"
    @keydown.enter="onSave"
  >
    <q-input v-model="task.summary" :label="$t('summary')" stack-label counter maxlength="40" outlined></q-input>

    <q-input v-model="task.description" :label="$t('description')" stack-label type="textarea" outlined></q-input>

    <div>Due date</div>
    <DatetimePicker v-model="task.due" :label="$t('dueDate')"></DatetimePicker>

    <div class="text-h6">{{ $t('duration') }}</div>
    <div>{{ $t('from') }}</div>
    <DatetimePicker
      v-model="task.start"
      label="Start"
      :date-only="wholeDay"
    />

    <div>{{ $t('to') }}</div>
    <DatetimePicker
      v-model="task.end"
      label="Due"
      :date-only="wholeDay"
    />
    <q-checkbox v-model="wholeDay">{{ $t('task_wholeDay') }}</q-checkbox>

    <h2 class="text-h6">{{ $t('recurrence') }}</h2>
    <RRulePicker
      v-model="task.rrule"
      :dtstart="task.start"
    />

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
import RRulePicker from 'components/toolkit/RRulePicker.vue'

const props = defineProps({
  editedTask: {
    type: Object as PropType<Task>,
    required: true
  }
})

const taskStore = useTaskStore()

const task = ref<Task>(props.editedTask)

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
