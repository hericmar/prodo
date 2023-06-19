<template>
  <q-page>
    <div class="row wrap">
      <div class="q-pa-lg col-12 col-md">
        <h1 class="text-h3">{{ $t('login_title') }}</h1>
      </div>
      <div class="q-pa-lg col-12 col-md">
        <q-card
          class="login-card q-pa-lg flex-center"
          flat
          bordered
        >
          <q-form
            @submit="onSubmit"
            v-model="accept"
            class="q-py-sm q-gutter-md flex column"
          >
            <q-input
              filled
              v-model="username"
              :placeholder="$t('username')"
              lazy-rules
              :rules="[ val => !!val || $t('empty') ]"
            />

            <q-input
              type="password"
              filled
              v-model="password"
              :placeholder="$t('password')"
              lazy-rules
              :rules="[ val => !!val || $t('empty') ]"
            />

            <q-btn :label="$t('login')" type="submit" color="primary"/>
          </q-form>
        </q-card>
      </div>
    </div>
  </q-page>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'

const accept = ref()
const username = ref()
const password = ref()

const onSubmit = () => {
  const authStore = useAuthStore()
  authStore.login(username.value, password.value)
    .then(() => {
      setTimeout(() => {
        router.push({ name: 'index' })
      }, 500)
    })
}
</script>
