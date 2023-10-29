<template>
  <div class="row items-center">
    <q-input
      ref="dateRef"
      v-model="date"
      :rules="[validateDate]"
      class="q-pr-sm"
      style="max-width: 150px"
      hide-bottom-space
      outlined
      dense
      @update:modelValue="onDateUpdate"
      @focusin="showDatePopup = true"
      @blur="onBlur"
      @keydown.esc="onEscDown"
    >
      <template v-slot:append>
        <q-icon name="event" />
      </template>
      <q-popup-proxy
        v-model="showDatePopup"
        persistent
        no-parent-event
        no-refocus
        no-focus
      >
        <q-date
          ref="popupDateRef"
          v-model="popupDate"
          minimal
          @update:modelValue="onPopupDateUpdate"
          @click.prevent
          @mousedown.prevent
        >
        </q-date>
      </q-popup-proxy>
    </q-input>

    <q-input
      ref="timeRef"
      v-model="time"
      mask="##:##"
      style="max-width: 100px"
      hide-bottom-space
      :rules="[validateTime]"
      outlined
      dense
      @update:modelValue="onUpdate"
    >
      <template v-slot:append>
        <q-icon name="access_time" />
      </template>
    </q-input>

    <div class="q-pl-md">
      <q-btn
        v-if="date"
        dense
        round
        unelevated
        size="sm"
        color="grey"
        icon="close"
        @click="onClear"
      >
      </q-btn>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { nextTick, ref, PropType } from 'vue'
import {
  formatDateLocal,
  formatTimeLocal,
  getDateFormatPattern,
  toDateModel
} from 'src/utils/datetime'
import { useQuasar } from 'quasar'
import { parse } from 'date-fns'

const props = defineProps({
  modelValue: {
    type: [Date, null] as PropType<Date | null>,
    required: true
  },
  label: {
    type: String,
    required: true
  },
  dateOnly: {
    type: Boolean,
    default: false
  }
})

const $q = useQuasar()
const locale = $q.lang.getLocale() || 'en-US'
const dateFormat = getDateFormatPattern(locale)

const time = ref('')
const timeRef = ref(null)
const date = ref('')
const dateRef = ref(null)
const popupDate = ref('')
const popupDateRef = ref(null)
const showDatePopup = ref(false)

const emit = defineEmits(['update:modelValue'])

if (props.modelValue) {
  date.value = formatDateLocal(props.modelValue, { dateOnly: true })
  time.value = formatTimeLocal(props.modelValue)
  popupDate.value = toDateModel(props.modelValue)
}

/*
watch(
  () => props.dateOnly,
  (dateOnly, _) => {
    model.value = formatDate(props.modelValue, props.dateOnly)
  }
)
 */

const validateDate = (date: string) => {
  if (!date) {
    return !time.value
  }

  try {
    const result = parse(date, dateFormat, new Date())
    // is valid
    return result instanceof Date && !isNaN(result.getTime())
  } catch (error) {
    return false
  }
}

const validateTime = (time: string) => {
  dateRef.value.validate()
  if (!time) {
    return true
  }
  return /^(?:[01]\d|2[0-3]):[0-5]\d$/.test(time)
}

const onClear = () => {
  time.value = ''
  date.value = ''
  emit('update:modelValue', null)
}

const onEscDown = (event: KeyboardEvent) => {
  if (showDatePopup.value) {
    showDatePopup.value = false
    event.stopPropagation()
  }
}

const onUpdate = () => {
  if (!validateTime(time.value) || !validateDate(date.value)) {
    return
  }

  const result = parse(date.value, dateFormat, new Date())
  if (time.value) {
    result.setHours(parseInt(time.value.substring(0, 2)))
    result.setMinutes(parseInt(time.value.substring(3)))
  }

  if (date.value === '' && time.value === '') {
    emit('update:modelValue', null)
  } else {
    emit('update:modelValue', result)
  }
}

const onDateUpdate = () => {
  const result = parse(date.value, dateFormat, new Date())
  nextTick(() => {
    // format date using YYYY/MM/DD
    popupDate.value = toDateModel(result)
    onUpdate()
  })
}

const onPopupDateUpdate = () => {
  // format date using locale format
  showDatePopup.value = false
  nextTick(() => {
    // Can be empty if user clicks once more on the date.
    date.value = Intl.DateTimeFormat(locale).format(new Date(popupDate.value))
    onUpdate()
  })
}

const onBlur = (e: FocusEvent) => {
  const calendarElementSelectorName = 'span.q-focus-helper'
  let emittedFromCalendar = false

  if (!popupDateRef.value || !dateRef.value) {
    return
  }

  popupDateRef.value.$el.querySelectorAll(calendarElementSelectorName).forEach((el: Node) => {
    if (el === e.relatedTarget) {
      emittedFromCalendar = true
    }
  })

  if (!emittedFromCalendar) {
    showDatePopup.value = false
  } else {
    dateRef.value.focus()
  }
}
</script>
