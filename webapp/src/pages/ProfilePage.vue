<template>
  <q-page
    class="page"
    padding
  >
    <h1 class="text-h4">{{ $t('profile') }}</h1>
    <q-card
      class="q-mb-lg"
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
      flat
    >
      <q-card-section>
        <h2 class="text-h6">Preferences</h2>
        <div>
          Language: <LangSelect display-label size="md" />
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { copyToClipboard } from 'quasar'
import api from 'src/api'
import LangSelect from 'components/toolkit/LangSelect.vue'

const link = ref('')

api.ical.get().then(response => {
  link.value = getLink(response.data.secret)
}).catch(() => {
  link.value = ''
})

const confirmLink = ref(false)
emitter.on('on-link', () => {
  confirmLink.value = true
})

const onCreateLink = () => {
  api.ical.create().then(() => {
    api.ical.get().then(response => {
      link.value = getLink(response.data.secret)
    })
  })
}

const onRemoveLink = () => {
  api.ical.delete().then(() => {
    link.value = ''
  })
}

const getLink = (secret: string) => {
  return `${window.location.protocol}//${window.location.host}/api/v1/calendar/subscription/${secret}`
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
