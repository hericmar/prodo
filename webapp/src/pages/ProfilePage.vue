<template>
  <q-page
    class="page q-pt-sm q-px-md"
  >
    <h1 class="text-h4">{{ $t('profile') }}</h1>

    <q-card
      class="responsive-card q-mb-lg"
      flat
    >
      <q-card-section>
        <h2 class="text-h6">{{ $t('subscription_header') }}</h2>
        <p>{{ $t('subscription_url_description') }}</p>
      </q-card-section>
      <q-card-section>
          <span v-if="link === ''" class="flex items-center">
            <q-btn icon="add" round flat @click="onCreateLink"></q-btn>
            {{ $t('subscription_generate') }}
          </span>
        <q-input v-else outlined v-model="link" :label="$t('subscription_url')" readonly>
          <template v-slot:after>
            <q-btn flat icon="content_copy" @click="onLinkCopy" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-actions align="center">
        <q-btn
          v-if="link !== ''"
          flat
          no-caps
          label="Remove"
          color="red"
          @click="onRemoveLink"
        />
      </q-card-actions>
    </q-card>

    <q-card
      class="responsive-card"
      flat
    >
      <q-card-section class="q-pb-lg">
        <h2 class="text-h6">Preferences</h2>
        <div>
          Language: <LangSelect class="q-ml-md" display-label size="md" />
        </div>
        <TimezonePicker
          v-model="timezone"
          @update:modelValue="useSettingsStore().updateSettings({ timezone: timezone })"
        />
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { copyToClipboard } from 'quasar'
import LangSelect from 'components/toolkit/LangSelect.vue'
import TimezonePicker from 'components/toolkit/TimezonePicker.vue'
import { useSettingsStore } from 'stores/settings-store'

const settingsStore = useSettingsStore()

const timezone = ref(settingsStore.timezone)

const getLink = (secret: string) => {
  return `${window.location.protocol}//${window.location.host}/api/v1/calendar/subscription/${secret}`
}

const link = computed(() => settingsStore.subscriptionSecret ? getLink(settingsStore.subscriptionSecret) : '')

const onCreateLink = () => {
  settingsStore.generateSubscription()
}

const onRemoveLink = () => {
  settingsStore.deleteSubscription()
}

const onLinkCopy = () => {
  copyToClipboard(link.value)
    .then(() => {
      // success!
    })
    .catch(() => {
      // fail
    })
}
</script>
