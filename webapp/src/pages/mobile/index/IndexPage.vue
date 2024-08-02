<template>
  <q-page
    class="q-pt-lg q-px-md"
  >
    <TaskList
      v-if="lists.length > 0"
      :list="lists[0]"
    />

    <h2 class="text-h6">My lists</h2>
    <q-card flat>
      <q-list separator>
        <q-item
          v-for="list in lists"
          :key="list.uid"
          clickable
          v-ripple
          @click="() => $router.push(`/list/${list.uid}`)"
        >
          <q-item-section>
            <q-item-label>
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
