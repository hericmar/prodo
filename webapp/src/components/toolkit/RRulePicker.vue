<template>
  <!-- https://jakubroztocil.github.io/rrule/ -->
  <q-select
    v-model="rruleType"
    :options="rruleTypeOptions"
    :option-label="option => $t(option)"
    outlined
  />

  <div v-if="rruleType === 'custom'">
    <div class="flex items-baseline">
      <span class="q-pr-sm">{{ $t('recurrence_once') }}</span>
      <q-input
        v-model.number="interval"
        class="q-pr-sm"
        type="number"
        dense
        outlined
        style="max-width: 100px"
        :rules="[val => val > 0 || $t('value_must_be_positive')]"
      />
      <q-select
        v-model="freq"
        :options="freqOptions"
        :option-label="option => $t(freqLabels[option])"
        dense
        outlined
        style="max-width: 100px"
      />
    </div>
    <div class="">
      Repeat on
      <q-option-group
        name=""
        model-value="value"
        v-model="byweekday"
        :options="byweekdayOptions"
        type="checkbox"
      />
    </div>
    <div>
      Ends
      <q-option-group
        v-model="ends"
        :options="endsOptions"
        type="radio"
      />
      <q-input
        v-if="ends === 'after'"
        v-model.number="count"
        type="number"
        dense
        outlined
        style="max-width: 100px"
        :rules="[val => val > 0 || $t('value_must_be_positive')]" />
      <DatetimePicker
        v-if="ends === 'on'"
        v-model="until"
        label="Due"
        date-only
      />
    </div>
  </div>
  <p class="text-grey-7"><i>{{ toText }}</i></p>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { RRule } from 'rrule'
import DatetimePicker from 'components/toolkit/DatetimePicker.vue'
import recurrence, { BYWEEKDAY_OPTIONS } from 'src/utils/recurrence'

const props = defineProps({
  modelValue: {
    type: String,
    required: true
  },
  dtstart: {
    type: Date,
    required: true
  }
})

const rruleType = ref(props.modelValue ? 'custom' : 'without_recurrence')
const rruleTypeOptions = [
  'without_recurrence',
  'daily',
  'weekly',
  'monthly',
  'yearly',
  'each_workday',
  'custom'
]

// eslint-disable-next-line vue/no-setup-props-destructure
const hasRRule = props.modelValue !== null && props.modelValue !== ''
const rruleOpts = hasRRule ? RRule.fromString(props.modelValue).options : null

const interval = ref<number>(
  rruleOpts?.interval ? rruleOpts.interval : 1
)

const freq = ref(
  rruleOpts?.freq !== undefined ? rruleOpts.freq : RRule.DAILY
)
// RRule.FREQUENCIES
const freqOptions = [RRule.DAILY, RRule.WEEKLY, RRule.MONTHLY, RRule.YEARLY]
const freqLabels = ['daily', 'weekly', 'monthly', 'yearly'].reverse()

const byweekday = ref(rruleOpts?.byweekday ? rruleOpts.byweekday : [])
const byweekdayOptions = BYWEEKDAY_OPTIONS

let endsValue = ''
console.log(rruleOpts)
if (rruleOpts?.count !== undefined && rruleOpts?.count !== null) {
  endsValue = 'after'
} else if (rruleOpts?.until !== undefined && rruleOpts?.until !== null) {
  endsValue = 'on'
}

const ends = ref(endsValue)
const endsOptions = [
  {
    label: 'never',
    value: ''
  },
  {
    label: 'after',
    value: 'after'
  },
  {
    label: 'on',
    value: 'on'
  }
]
const count = ref(rruleOpts?.count !== undefined ? rruleOpts.count : 1)
const until = ref(rruleOpts?.until !== undefined && rruleOpts?.until !== null ? rruleOpts.until : new Date())

const emit = defineEmits(['update:modelValue'])

const toText = computed(() => {
  let rule = null
  let options = null
  if (rruleType.value === 'without_recurrence') {
    emit('update:modelValue', null)
    return ''
  } else if (rruleType.value !== 'custom') {
    options = recurrence[rruleType.value]()
  } else {
    options = {
      freq: freq.value,
      interval: interval.value,
      byweekday: byweekday.value
    }
    if (ends.value === 'after') {
      (options as any).count = count.value
    } else if (ends.value === 'on') {
      (options as any).until = until.value
    }
  }
  rule = new RRule(options)
  emit('update:modelValue', rule.toString())

  return rule.toText()
})
</script>
