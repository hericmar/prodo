<template>
  <h2
    v-if="props.readonly || !editing"
    class="text-h6 q-my-sm"
    @click="onClick"
  >
    {{ props.label }}
  </h2>
  <!-- v-if="!props.readonly && editing" -->
  <q-input
    ref="input"
    v-model="value"
    class="custom-input text-h6 q-my-sm"
    hide-hint
    @keydown.enter="onBlur"
    @blur="onBlur"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'

// define with defaults
const props = withDefaults(defineProps<{
  readonly?: boolean
  label: string
  customClass?: string
}>(), {
  customClass: ''
})

const emit = defineEmits(['update:modelValue'])

const editing = ref<boolean>(false)
const value = ref<string>(props.label)
const input = ref<HTMLInputElement | null>(null)

const onClick = () => {
  editing.value = true
  setTimeout(() => {
    input.value?.focus()
  }, 0)
}

const onBlur = () => {
  editing.value = false
  emit('update:modelValue', value.value)
}
</script>

<style>
.custom-input input {
  align-self: flex-start;
  padding-top: 0;
}
</style>
