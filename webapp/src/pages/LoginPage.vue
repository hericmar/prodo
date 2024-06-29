<template>
  <q-page
    class="page q-mt-xl"
  >
    <div class="row wrap">
      <div class="q-pa-lg col-12 col-md">
        <h1 class="text-h4">{{ $t('login_title') }}</h1>
      </div>
      <div class="q-pa-lg col-12 col-md">
        <q-card
          class="q-pa-lg flex-center"
          flat
          bordered
        >
          <q-form
            @submit="onSubmit"
            v-model="accept"
            class="q-py-sm q-gutter-md flex column"
          >
            <q-input
              v-model="username"
              :label="$t('username')"
              lazy-rules
              :rules="[ val => !!val || $t('empty') ]"
            />

            <q-input
              type="password"
              v-model="password"
              :label="$t('password')"
              lazy-rules
              :rules="[ val => !!val || $t('empty') ]"
            />

            <p>{{ message }}</p>

            <q-btn
              :label="$t('login')"
              type="submit"
              no-caps
              color="primary"
            />
          </q-form>
        </q-card>
      </div>
    </div>
  </q-page>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from 'stores/auth-store'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const i18n = useI18n()

const accept = ref()
const username = ref()
const password = ref()

const message = ref()

const router = useRouter()

const onSubmit = () => {
  const authStore = useAuthStore()
  authStore.login(username.value, password.value)
    .then(() => {
      setTimeout(() => {
        message.value = ''
        router.push({ name: 'index' })
      }, 1000)
    })
    .catch(() => {
      message.value = i18n.t(authStore.message)
    })
}
</script>
