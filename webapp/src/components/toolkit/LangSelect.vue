<template>
  <q-btn-dropdown
    class="prodo-lang-select q-px-sm"
    :size="props.size"
    icon="language"
    no-caps
    :label="props.displayLabel ? localeOptions.find(l => l.value === locale)?.label : ''"
    flat
    rounded
  >
    <q-list>
      <q-item
        v-for="locale in localeOptions"
        :key="locale.value"
        clickable v-close-popup @click="onItemClick(locale.value)">
        <q-item-section>
          <q-item-label>{{ locale.label }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-list>
  </q-btn-dropdown>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from 'stores/settings-store'

const props = defineProps({
  displayLabel: {
    type: Boolean,
    default: false
  },
  size: {
    type: String,
    default: 'sm'
  }
})

const { locale } = useI18n({ useScope: 'global' })
const localeOptions = [
  { label: 'English', value: 'en-US' },
  { label: 'Čeština', value: 'cs-CZ' }
]

const onItemClick = (value: string) => {
  locale.value = value
  useSettingsStore().updateSettings({
    preferredLocale: value
  })
}
</script>

<style>
.prodo-lang-select span[class="block"] {
  font-weight: 400 !important;
}
</style>
