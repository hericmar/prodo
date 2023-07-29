<template>
  <q-input
    v-model="model"
    outlined
    dense
    clearable
    @update:modelValue="onUpdate"
  >
    <template v-slot:prepend>
      <q-icon name="event" />
    </template>
    <q-popup-proxy class="flex flex-center">
      <q-date
        v-model="model"
        :mask="props.dateOnly ? 'YYYY-MM-DD' : 'YYYY-MM-DD HH:mm'"
        class="q-mr-sm"
        @update:modelValue="onUpdate"
      >
      </q-date>
      <q-time
        v-if="!props.dateOnly"
        v-model="model"
        mask="YYYY-MM-DD HH:mm"
        @update:modelValue="onUpdate"
      >
      </q-time>
    </q-popup-proxy>
  </q-input>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import { formatDate } from 'src/utils/datetime'

const props = defineProps({
  modelValue: {
    type: Date,
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

const model = ref(formatDate(props.modelValue, props.dateOnly))

const emit = defineEmits(['update:modelValue'])

watch(
  () => props.dateOnly,
  (dateOnly, _) => {
    model.value = formatDate(props.modelValue, props.dateOnly)
  }
)

const onUpdate = () => {
  console.log('onUpdate', model.value)
  if (model.value === null) {
    emit('update:modelValue', null)
    return
  }
  emit('update:modelValue', new Date(model.value))
}
</script>
