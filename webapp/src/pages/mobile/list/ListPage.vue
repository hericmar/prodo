<template>
  <q-page
    class="q-py-lg q-px-md"
  >
    <TaskList
      v-if="list"
      :list="list"
    />
  </q-page>
</template>

<script setup lang="ts">
import { useTaskStore } from 'stores/task-store'
import { useRoute } from 'vue-router'
import { computed } from 'vue'
import TaskList from 'components/tasks/TaskList.vue'

const taskStore = useTaskStore()

const route = useRoute()

const list = computed(() => taskStore.lists.find(list => list.uid === route.params.uid))

// TODO: workaround
if (taskStore.lists.length === 0) {
  taskStore.init()
}
</script>
