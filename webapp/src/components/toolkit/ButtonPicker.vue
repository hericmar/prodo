<template>
  <div class="q-gutter-sm q-mb-md">
    <q-btn
      v-for="(item, index) in options" :key="index"
      @click="onClick(index)"
      class="q-px-sm"
      :label="item"
      :color="selected.includes(index) ? 'primary' : 'grey-5'"
      no-caps
      dense
    />
  </div>
</template>

<script lang="ts" setup>
import { PropType, ref, defineProps, defineEmits } from 'vue'

const props = defineProps({
  options: {
    type: Array,
    required: true
  },
  modelValue: {
    type: Array,
    required: true
  }
})

const selected = ref(props.modelValue)

const emit = defineEmits(['update:modelValue'])

const onClick = (index: number) => {
  if (selected.value.includes(index)) {
    selected.value = selected.value.filter(d => d !== index)
  } else {
    selected.value.push(index)
  }
  emit('update:modelValue', selected)
}
</script>
