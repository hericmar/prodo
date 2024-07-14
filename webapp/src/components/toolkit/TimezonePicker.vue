<template>
  <q-select
    v-model="model"
    use-input
    input-debounce="0"
    label="Timezone"
    :options="options"
    @filter="filterFn"
    style="width: 250px"
    @update:modelValue="emit('update:modelValue', model)"
  >
    <template v-slot:no-option>
      <q-item>
        <q-item-section class="text-grey">
          No results
        </q-item-section>
      </q-item>
    </template>
  </q-select>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// with defaults
const props = defineProps<{
  modelValue: string | null
}>()
const emit = defineEmits(['update:modelValue'])

const stringOptions = Intl.supportedValuesOf('timeZone')

const model = ref<string | null>(props.modelValue)
const options = ref<string[]>(stringOptions)

const filterFn = (val: string, update: any) => {
  if (val === '') {
    update(() => {
      options.value = stringOptions

      // here you have access to "ref" which
      // is the Vue reference of the QSelect
    })
    return
  }

  update(() => {
    const needle = val.toLowerCase()
    options.value = stringOptions.filter((v: string) => v.toLowerCase().indexOf(needle) > -1)
  })
}
</script>
