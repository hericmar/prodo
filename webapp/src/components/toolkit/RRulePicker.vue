<template>
  <!-- https://jakubroztocil.github.io/rrule/ -->
  <q-select
    v-model="rruleType"
    :options="rruleTypeOptions"
    :option-label="option => $t(option)"
    outlined
  />

  <div
    v-if="rruleType === 'custom'"
    class="q-mb-md"
  >
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

    <div class="q-mb-lg">
      <div class="q-mb-sm">Repeat on</div>
      <RepeatPicker v-model="byweekday" />
    </div>

    <div class="q-mb-sm">Ends</div>
    <q-option-group
      v-model="ends"
      class="q-mb-md"
      :options="endsOptions"
      type="radio"
    />
    <div v-if="ends === 'after'" class="flex items-baseline">
      <q-input
        v-model.number="count"
        class="q-mr-sm"
        type="number"
        dense
        outlined
        style="max-width: 100px"
        :rules="[val => val > 0 || $t('value_must_be_positive')]"
      />
      <span>{{ $t('recurrences') }}</span>
    </div>
    <DatetimePicker
      v-if="ends === 'on'"
      v-model="until"
      label="Due"
      date-only
    />
  </div>
  <p class="text-grey-7"><i>{{ toText }}</i></p>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { RRule } from 'rrule'
import DatetimePicker from 'components/toolkit/DatetimePicker.vue'
import recurrence from 'src/utils/recurrence'
import RepeatPicker from 'components/toolkit/RepeatPicker.vue'

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

// Select recurrence type
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
const opts = hasRRule ? RRule.fromString(props.modelValue).options : null

// Custom recurrence options

// Frequency of recurrence
const interval = ref<number>(
  opts?.interval ? opts.interval : 1
)

const freq = ref(
  opts?.freq !== undefined ? opts.freq : RRule.DAILY
)
// RRule.FREQUENCIES
const freqOptions = [RRule.DAILY, RRule.WEEKLY, RRule.MONTHLY, RRule.YEARLY]
const freqLabels = ['daily', 'weekly', 'monthly', 'yearly'].reverse()

// Repeat on
const byweekday = ref(opts?.byweekday ? opts.byweekday : [])

// Ends on and after (count and until)
let endsValue = ''
if (opts?.count !== undefined && opts?.count !== null) {
  endsValue = 'after'
} else if (opts?.until !== undefined && opts?.until !== null) {
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
const count = ref(opts?.count !== undefined ? opts.count : 1)
const until = ref(opts?.until !== undefined && opts?.until !== null ? opts.until : new Date())

const emit = defineEmits(['update:modelValue'])

// Emit updated recurrence rule
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
  const ruleString = rule.toString().replace('RRULE:', '')

  emit('update:modelValue', ruleString)

  return rule.toText()
})
</script>
