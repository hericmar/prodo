<template>
  <q-layout view="lHh Lpr f" class="flex">
    <q-header class="prodo-header flex-center">
      <q-toolbar
        flat
        rounded
      >
        <q-toolbar-title>
          <q-btn
            :to="{ name: 'landing' }" flat
          >
            Prodo
          </q-btn>
        </q-toolbar-title>
        <div class="q-gutter-sm">
          <ThemeButton />
          <LangSelect />
          <LoginButton />
        </div>
      </q-toolbar>
    </q-header>
    <q-page-container class="prodo-container">
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import ThemeButton from 'components/toolkit/ThemeButton.vue'
import LangSelect from 'components/toolkit/LangSelect.vue'
import LoginButton from 'components/toolkit/LoginButton.vue'
</script>

<style lang="scss">
.prodo-header {
  background-color: $primary !important;
}

@media (min-width: $breakpoint-xs) {
  .prodo-header {
    width: 80vw;
    margin-top: 8px;
    margin-left: auto;
    margin-right: auto;
    padding-top: 4px;
    padding-bottom: 4px;
    border-radius: 6px;
  }
}

@media (min-width: $breakpoint-xs) {
  .prodo-container {
    width: 80vw;
    margin-left: auto;
    margin-right: auto;
  }
}
</style>
