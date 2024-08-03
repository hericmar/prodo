<template>
  <div class="flex justify-between full-width q-px-xl q-pt-sm q-pb-lg">
    <q-btn
      class="text-body1 text-bold"
      flat
      rounded
      no-caps
      label="Cancel"
      color="blue-7"
      @click="onClose"
    />
    <q-btn
      class="text-body1 text-bold"
      flat
      rounded
      no-caps
      label="Save"
      color="blue-7"
      @click="onSave"
    />
  </div>
</template>

<script setup lang="ts">
import emitter from 'src/plugins/mitt'
import { TaskEvent } from 'stores/task-store'
import { useRouter } from 'vue-router'

const router = useRouter()

const onClose = () => {
  emitter.emit(TaskEvent.OnUpdateCancel)
  router.back()
}

const onSave = () => {
  emitter.emit(TaskEvent.OnUpdateSave)
  router.back()
}
</script>
