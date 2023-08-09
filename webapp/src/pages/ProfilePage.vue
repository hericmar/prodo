<template>
  <q-page
    class="page"
    padding
  >
    <h1 class="text-h4">{{ $t('profile') }}</h1>
    <q-card>
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
      <q-card-actions>
        <q-btn flat label="Remove" color="red" @click="onRemoveLink" v-if="link !== ''" />
      </q-card-actions>
    </q-card>
  </q-page>
</template>

<script setup>
import { ref } from 'vue'
import emitter from 'src/plugins/mitt'
import { copyToClipboard } from 'quasar'
import api from 'src/api'
import { BASE_URL } from 'src/boot/axios'

const link = ref('')

api.ical.get().then((res) => {
  link.value = BASE_URL + '/api/v1/ical/' + res.data.secret
}).catch(() => {
  link.value = ''
})

const confirmLink = ref(false)
emitter.on('on-link', () => {
  confirmLink.value = true
})

const onCreateLink = () => {
  api.ical.create().then((res) => {
    api.ical.get().then((res) => {
      link.value = BASE_URL + '/api/v1/ical/' + res.data.secret
    })
  })
}

const onRemoveLink = () => {
  api.ical.delete().then(() => {
    link.value = ''
  })
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
