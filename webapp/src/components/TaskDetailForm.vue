<template>
  <q-form class="q-py-sm q-gutter-md flex column">
    <q-input v-model="task.summary" :label="$t('summary')" stack-label counter maxlength="40" outlined></q-input>

    <q-input v-model="task.description" :label="$t('description')" stack-label type="textarea" outlined></q-input>

    <!-- https://github.com/jakubroztocil/rrule -->
    <!-- TODO rrule input https://codepen.io/theeternalsw0rd/pen/JLMjGx -->

    <div class="q-pa-md q-gutter-sm">
      <q-btn flat label="Cancel" color="primary" @click="onClose" />
      <q-btn flat label="Save" color="primary" @click="onSave" />
    </div>
  </q-form>
</template>

<script lang="ts" setup>
import { Task, useTaskStore } from 'stores/task-store'
import { PropType, ref } from 'vue'
import emitter from 'src/plugins/mitt'

const props = defineProps({
  editedTask: {
    type: Object as PropType<Task>,
    required: true
  }
})

const taskStore = useTaskStore()

const task = ref(props.editedTask)

const onClose = () => {
  emitter.emit('on-edit-close', {})
}

const onSave = () => {
  taskStore.update(task.value)
    .then(() => {
      onClose()
    })
}
</script>
