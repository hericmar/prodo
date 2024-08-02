<template>
  <q-btn
    flat
    rounded
    no-caps
    size="md"
    @click="router.back()"
  >
    <q-icon
      name="arrow_back_ios"
      size="20px"
    />
    Back
  </q-btn>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
</script>
