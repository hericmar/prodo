<template>
  <q-layout view="lHh Lpr fFf">
    <NavigationHeader>
      <router-view name="header" />
    </NavigationHeader>

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-footer
      class="prodo-mobile-toolbar flex justify-between full-width"
    >
      <router-view name="toolbar" />
    </q-footer>
  </q-layout>
</template>

<script lang="ts" setup>
import NavigationHeader from 'components/NavigationHeader.vue'
import emitter from 'src/plugins/mitt'
import { useRouter } from 'vue-router'

const router = useRouter()

// TODO: move somewhere else
emitter.on('on-edit', (e: any) => {
  router.push({
    name: 'task',
    params: {
      uid: e.task.uid
    }
  })
})
</script>

<style lang="scss">
footer {
  margin-bottom: -1px;
}

.body--light footer {
  // background-color: rgba(255, 255, 255, 0.7) !important;
  background-color: rgb(255, 255, 255) !important;
  backdrop-filter: blur(10px);
}

.body--dark footer {
  // background-color: rgba(0, 0, 0, 0.7) !important;
  background-color: rgb(0, 0, 0) !important;
  backdrop-filter: blur(10px);
}

.body--light .prodo-mobile-toolbar {
  border-top: 1px solid $separator-color;
}

.body--dark .prodo-mobile-toolbar {
  border-top: 1px solid $separator-dark-color;
}

.prodo-mobile-toolbar:not(:empty) {
  padding-bottom: 8px;
}
</style>
