<template>
  <q-page
    class="q-py-lg q-px-md"
  >
    <TaskList
      v-if="lists.length > 0"
      :list="taskStore.lists[0]"
    />

    <h2 class="text-h6">My lists</h2>
    <q-card
      class="q-mb-md"
      flat
    >
      <q-list separator>
        <q-item
          v-for="list in lists"
          :key="list.uid"
          clickable
          v-ripple
          @click="() => $router.push(`/list/${list.uid}`)"
        >
          <q-item-section>
            <q-item-label class="text-body1">
              {{ list.name }}
            </q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-card>
  </q-page>
</template>

<script setup lang="ts">
import { useTaskStore } from 'stores/task-store'
import { computed } from 'vue'
import TaskList from 'components/tasks/TaskList.vue'

const taskStore = useTaskStore()
taskStore.init()

const lists = computed(() => taskStore.lists.filter(list => !list.isVirtual))
</script>
