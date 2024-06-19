<template>
  <q-btn
    class=""
    @click="onClick"
    flat
    round
    size="sm"
    :icon="$q.dark.isActive ? 'dark_mode' : 'light_mode'"
  >

  </q-btn>
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
import { useSettingsStore } from 'stores/settings-store'
import { onMounted } from 'vue'

const $q = useQuasar()

onMounted(() => {
  const preferDarkMode = useSettingsStore().preferDarkMode
  if (preferDarkMode !== undefined) {
    $q.dark.set(preferDarkMode)
  }
})

const onClick = () => {
  $q.dark.toggle()
  useSettingsStore().updateSettings({ preferDarkMode: $q.dark.isActive })
  const themeColor = document.querySelector('meta[name="theme-color"]')
  if (themeColor) {
    themeColor.setAttribute('content', '#123456')
  }
}
</script>
