<template>
  <q-layout view="lHh Lpr fff">
    <NavigationHeader />

    <q-page-container
      class="q-pa-md"
    >
      <router-view />
    </q-page-container>

    <q-footer class="q-pt-xl q-pb-lg q-px-md bg-white text-grey-5 flex flex-center">
      <div>
        @ {{ new Date().getFullYear() }} {{ author }}, version {{ version }}
      </div>
    </q-footer>
  </q-layout>
</template>

<script lang="ts" setup>
import packageJson from '../../package.json'
import NavigationHeader from 'components/NavigationHeader.vue'

const author = packageJson.author
const version = packageJson.version
</script>
