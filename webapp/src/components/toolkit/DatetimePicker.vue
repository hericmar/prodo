<template>
  <div style="max-width: 300px">
    <q-input outlined v-model="dateStr" @update:modelValue="onUpdate()" :label="props.label">
      <template v-slot:prepend>
        <q-icon name="event" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-date
              v-model="dateStr"
              @update:modelValue="onUpdate()"
              :mask="props.dateOnly ? 'YYYY-MM-DD' : 'YYYY-MM-DD HH:mm'"
            >
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Close" color="primary" flat />
              </div>
            </q-date>
          </q-popup-proxy>
        </q-icon>
      </template>

      <template v-if="!dateOnly" v-slot:append>
        <q-icon name="access_time" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-time
              v-model="dateStr"
              @update:modelValue="onUpdate()"
              :mask="props.dateOnly ? 'YYYY-MM-DD' : 'YYYY-MM-DD HH:mm'"
              format24h
            >
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Close" color="primary" flat />
              </div>
            </q-time>
          </q-popup-proxy>
        </q-icon>
      </template>
    </q-input>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'

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

const emit = defineEmits(['update:modelValue'])

let formattedDate = null
if (props.modelValue) {
  const splitted = props.modelValue.toISOString().split('T')
  formattedDate = splitted[0] + ' ' + splitted[1].substring(0, 5)
}

const dateStr = ref(formattedDate)

const onUpdate = () => {
  console.log('onUpdate')
  emit('update:modelValue', new Date(dateStr.value))
}

watch(() => props.modelValue, (newValue, _) => {
  if (newValue === null) {
    dateStr.value = null
  } else {
    const splitted = newValue.toISOString().split('T')
    dateStr.value = splitted[0] + ' ' + splitted[1].substring(0, 5)
  }
})

watch(
  () => props.dateOnly,
  (newValue, _) => {
    if (newValue) {
      dateStr.value = dateStr.value.split(' ')[0]
    } else {
      dateStr.value = `${dateStr.value} 00:00`
    }
    onUpdate()
  }
)
</script>
