<template>
  <q-header>
    <q-toolbar class="bg-indigo justify-between">
      <div class="prodo-header-title">
        <slot></slot>

        <q-toolbar-title>
          <q-btn
            v-if="isAuthenticated"
            :to="{ name: 'index' }"
            flat
            rounded
          >
            Prodo
          </q-btn>
          <q-btn
            v-else
            :to="{ name: 'login' }"
            flat
            rounded
          >
            Prodo
          </q-btn>
        </q-toolbar-title>
      </div>

      <div>
        <ThemeButton class="q-mr-sm" />

        <q-btn-dropdown
          v-if="isAuthenticated"
          :label="$t('profile')"
          no-caps
          rounded
          unelevated
          flat
        >
          <q-list>
            <q-item clickable to="profile">
              <q-item-section>{{ $t('settings') }}</q-item-section>
              <q-item-section avatar>
                <q-icon name="settings" size="20px" />
              </q-item-section>
            </q-item>
            <q-item
              clickable
              v-ripple
              @click="onLogout"
            >
              <q-item-section>{{ $t('logout') }}</q-item-section>
              <q-item-section avatar>
                <q-icon name="logout" size="20px" />
              </q-item-section>
            </q-item>
          </q-list>
        </q-btn-dropdown>
      </div>
    </q-toolbar>
  </q-header>
</template>

<script setup lang="ts">
import ThemeButton from 'components/toolkit/ThemeButton.vue'
import { computed } from 'vue'
import { useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'

const authStore = useAuthStore()

const isAuthenticated = computed(() => authStore.isAuthenticated)

const onLogout = async () => {
  await authStore.logout()
  router.push({ name: 'login' })
}
</script>

<style>
.prodo-header-title > :nth-child(2) {
  display: none;
}
</style>
