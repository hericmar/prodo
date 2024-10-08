<template>
  <div class="row items-center">
    <q-input
      :type="dateInputType"
      color="blue-6"
      ref="dateRef"
      v-model="date"
      class="date-input q-pr-sm"
      hide-bottom-space
      outlined
      dense
      @update:modelValue="onUpdate"
      @focusin="onFocus"
      @blur="onBlur"
      @keydown.esc="onEscDown"
    >
      <q-popup-proxy
        v-model="showDatePopup"
        persistent
        no-parent-event
        no-refocus
        no-focus
      >
        <q-date
          ref="popupDateRef"
          v-model="date"
          mask="YYYY-MM-DD"
          minimal
          no-unset
          @update:modelValue="onUpdate"
          @click.prevent
          @mousedown.prevent
        />
      </q-popup-proxy>
    </q-input>

    <q-input
      v-if="!props.dateOnly"
      class="time-input"
      color="blue-6"
      ref="timeRef"
      v-model="time"
      mask="##:##"
      type="time"
      hide-bottom-space
      outlined
      dense
      @click="onTimeClick"
      @update:modelValue="onUpdate"
    />

    <q-btn
      v-if="date"
      class="q-ml-md"
      dense
      round
      unelevated
      size="sm"
      color="grey"
      icon="close"
      @click="onClear"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref, useTemplateRef } from 'vue'
import { formatTime } from 'src/utils/datetime'
import { QInput } from 'quasar'

interface Props {
  modelValue: Date | null;
  label: string;
  dateOnly?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  dateOnly: false
})

const time = ref('')
const timeRef = useTemplateRef('timeRef')

const date = ref('')
const dateRef = useTemplateRef('dateRef')
const dateInputType = ref<'text' | 'date'>('text')

const popupDateRef = useTemplateRef('popupDateRef')
const showDatePopup = ref(false)

const emit = defineEmits(['update:modelValue'])

if (props.modelValue) {
  dateInputType.value = 'date'
  date.value = props.modelValue.toISOString().substring(0, 10)
  time.value = formatTime(props.modelValue)
}

const onClear = () => {
  time.value = ''
  date.value = ''
  dateInputType.value = 'text'
  emit('update:modelValue', null)
}

const onEscDown = (event: KeyboardEvent) => {
  if (showDatePopup.value) {
    showDatePopup.value = false
    event.stopPropagation()
  }
}

const onUpdate = () => {
  if (date.value) {
    const result = new Date(date.value)
    result.setHours(0)
    result.setMinutes(0)

    if (time.value) {
      const hours = parseInt(time.value.substring(0, 2))
      const minutes = parseInt(time.value.substring(3, 5))

      result.setHours(hours)
      result.setMinutes(minutes)
    }
    emit('update:modelValue', result)
  } else {
    emit('update:modelValue', null)
  }
}

const onFocus = () => {
  dateInputType.value = 'date'
  showDatePopup.value = true
}

const onTimeClick = () => {
  const input = timeRef.value.$el.querySelector('input')
  if ('showPicker' in input) {
    input.showPicker()
  }
}

const onBlur = (e: FocusEvent) => {
  if (!date.value) {
    dateInputType.value = 'text'
  }

  const emittedFromPopup = Array.from(popupDateRef.value.$el.querySelectorAll('span.q-focus-helper'))
    .includes(e.relatedTarget)

  if (!emittedFromPopup) {
    showDatePopup.value = false
  } else {
    dateRef.value.focus()
  }
}
</script>

<style lang="scss" scoped>
.date-input {
  width: 120px;
}

.time-input {
  width: 100px;
}
</style>
