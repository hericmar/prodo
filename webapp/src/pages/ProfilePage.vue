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
        <span v-if="link === ''" class="flex items-center">
          <q-btn icon="add" round flat @click="onCreateLink"></q-btn>
          {{ $t('subscription_generate') }}
        </span>
        <div v-else>
          <p>{{ $t('subscription_url_description') }}</p>
          <q-input outlined v-model="link" :label="$t('subscription_url')" readonly>
            <template v-slot:after>
              <q-btn flat icon="content_copy" @click="onLinkCopy" />
            </template>
          </q-input>
            <div class="q-mt-md text-grey-14">
              Last sync at: {{ formatDateLocal(settingsStore.calendarLastSync) || 'Never' }}
            </div>
        </div>
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
      class="responsive-card q-mb-lg"
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

    <q-card
      class="responsive-card"
      flat
    >
      <q-card-section class="q-pb-lg">
        <h2 class="text-h6">Data</h2>
        <h3 class="text-body1">Archived Lists</h3>
        <q-list>
          <q-item
            v-for="list in archivedLists"
            :key="list.uid"
            v-ripple
            @click="() => $router.push(`/list/${list.uid}`)"
          >
            <q-item-section>
              <q-item-label class="">
                {{ list.name }}
              </q-item-label>
            </q-item-section>

            <q-item-section top side>
              <q-btn
                flat
                dense
                rounded
                no-caps
                @click="taskStore.updateList(list.uid, { is_archived: false })"
              >
                Unarchive
              </q-btn>
            </q-item-section>
          </q-item>
        </q-list>
        <p v-if="archivedLists.length === 0">
          None
        </p>
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
import { useTaskStore } from 'stores/task-store'
import { formatDateLocal } from 'src/utils/datetime'

const settingsStore = useSettingsStore()
const taskStore = useTaskStore()
taskStore.init()

const timezone = ref(settingsStore.timezone)

const getLink = (secret: string) => {
  return `${window.location.protocol}//${window.location.host}/api/v1/calendar/subscription/${secret}`
}

const link = computed(() => settingsStore.subscriptionSecret ? getLink(settingsStore.subscriptionSecret) : '')

const archivedLists = computed(() => taskStore.lists.filter(list => list.is_archived))

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
