<template>
  <q-form
    class="q-py-sm q-gutter-md flex column"
    @keydown.enter="onEnterDown"
  >
    <q-input
      v-model="task.summary"
      :label="$t('summary')"
      stack-label
      counter
      maxlength="60"
      outlined
    />

    <q-input v-model="task.description" :label="$t('description')" stack-label type="textarea" outlined />

    <div>Due date</div>
    <DatetimePicker v-model="task.due" :label="$t('dueDate')"></DatetimePicker>

    <h2 class="text-h6">{{ $t('duration') }}</h2>
    <div>{{ $t('from') }}</div>
    <DatetimePicker
      v-model="task.start"
      label="Start"
      :date-only="wholeDay"
      @update:modelValue="onStartChange"
    />

    <div>{{ $t('to') }}</div>
    <DatetimePicker
      v-model="task.end"
      label="Due"
      :date-only="wholeDay"
    />
    <div>
      <q-checkbox v-model="wholeDay" @click="onWholeDayClick">{{ $t('task_wholeDay') }}</q-checkbox>
    </div>

    <h2 class="text-h6">{{ $t('priority') }}</h2>
    <q-btn-toggle
      v-model="task.priority"
      spread
      :options="[
        { label: $t('low'), value: 9 },
        { label: $t('medium'), value: 5 },
        { label: $t('high'), value: 1 }
      ]"
    />

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
import ButtonPicker from 'components/toolkit/ButtonPicker.vue'

const props = defineProps({
  editedTask: {
    type: Object as PropType<Task>,
    required: true
  }
})

const taskStore = useTaskStore()

const task = ref<Task>(props.editedTask)

// handle from and to fields
const wholeDay = ref<boolean>(task.value.start ? isTimeSet(task.value.start) : false)

const onStartChange = (value: Date) => {
  onWholeDayClick()
}

const onWholeDayClick = () => {
  if (wholeDay.value && task.value.start) {
    task.value.end = task.value.start
  }
}

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
