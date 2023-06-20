<template>
  <q-item>
    <q-item-section side>
      <q-checkbox v-model="completed" />
    </q-item-section>

    <q-item-section>
      <q-item-label>{{ props.task.summary }}</q-item-label>
    </q-item-section>

    <q-item-section side>
      <div class="text-grey-8 q-gutter-xs">
        <q-btn size="12px" flat dense round icon="more_vert">
          <q-menu>
            <q-list>
              <q-item clickable v-close-popup>
                <q-item-section>Edit</q-item-section>
              </q-item>
              <q-item clickable v-close-popup>
                <q-item-section
                  @click="onDelete"
                >Delete</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
      </div>
    </q-item-section>
  </q-item>
</template>

<script lang="ts" setup>
import { Task } from 'stores/task-store'
import { ref } from 'vue'
import emitter from 'src/plugins/mitt'

interface Props {
  task: Task
}

const emit = defineEmits(['onDelete'])

const props = defineProps<Props>()

const completed = ref(props.task.completed !== null)

const onDelete = () => {
  emitter.emit('on-delete', { task: props.task })
}
</script>
