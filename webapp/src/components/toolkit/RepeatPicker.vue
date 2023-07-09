<template>
  <div class="q-gutter-sm q-mb-md">
    <q-btn
      v-for="day in byweekdayOptions" :key="day.value"
      @click="onClick(day.value)"
      class="q-px-sm"
      :label="day.label.substring(0, 2)"
      :color="byweekday.includes(day.value) ? 'primary' : 'grey-5'"
      no-caps
      dense
    />
  </div>
</template>

<script lang="ts" setup>
import { PropType, ref } from 'vue'
import { BYWEEKDAY_OPTIONS } from 'src/utils/recurrence'

const props = defineProps({
  // byweekday
  modelValue: {
    type: Array as PropType<number[]>,
    required: true
  }
})

const byweekday = ref(props.modelValue)
const byweekdayOptions = BYWEEKDAY_OPTIONS

const emit = defineEmits(['update:modelValue'])

const onClick = (day: number) => {
  if (byweekday.value.includes(day)) {
    byweekday.value = byweekday.value.filter(d => d !== day)
  } else {
    byweekday.value.push(day)
  }
  emit('update:modelValue', byweekday.value)
}
</script>
