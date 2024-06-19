<template>
  <q-btn
    :to="{ name: (!isAuthenticated ? 'login' : 'index')}"
    no-caps
    rounded
    outline
  >
    {{ $t(!isAuthenticated ? 'login' : 'toApp') }}
    <q-icon
      name="arrow_forward"
      class="q-pl-sm"
      size="16px"
    ></q-icon>
  </q-btn>
</template>

<script setup lang="ts">
import { useAuthStore } from 'stores/auth-store'
import { computed } from 'vue'

const authStore = useAuthStore()
const isAuthenticated = computed(() => authStore.isAuthenticated)

</script>
