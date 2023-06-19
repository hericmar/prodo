<template>
  <q-page
    class="row items-center justify-evenly"
  >
    <q-list>
      <q-item v-for="todo in todos" :key="todo.id">
        <q-item-section top side>
          <q-checkbox />
        </q-item-section>

        <q-item-section>
          <q-item-label>{{ todo.content }}</q-item-label>
        </q-item-section>

        <q-item-section top side>
          <div class="text-grey-8 q-gutter-xs">
            <q-btn size="12px" flat dense round icon="more_vert" />
          </div>
        </q-item-section>
      </q-item>
    </q-list>
    <example-component
      title="Example component"
      active
      :todos="todos"
      :meta="meta"
    ></example-component>
  </q-page>
</template>

<script setup lang="ts">
import { Todo, Meta } from 'components/models'
import ExampleComponent from 'components/ExampleComponent.vue'
import { ref, computed } from 'vue'
import { useAuthStore } from 'stores/auth-store'

const isAuthenticated = computed(() => useAuthStore().isAuthenticated)
console.log(isAuthenticated.value)

const todos = ref<Todo[]>([
  {
    id: 1,
    content: 'ct1'
  },
  {
    id: 2,
    content: 'ct2'
  },
  {
    id: 3,
    content: 'ct3'
  },
  {
    id: 4,
    content: 'ct4'
  },
  {
    id: 5,
    content: 'ct5'
  }
])
const meta = ref<Meta>({
  totalCount: 1200
})
</script>
