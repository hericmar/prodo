<template>
  <q-page
    class="row items-center justify-evenly"
  >
    <TaskList />
  </q-page>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTaskStore } from 'stores/task-store'
import TaskList from 'components/TaskList.vue'
</script>
