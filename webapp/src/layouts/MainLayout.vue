<template>
  <q-layout view="lHh Lpr f">
    <q-header>
      <q-toolbar class="bg-indigo">
        <!--
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />
        -->

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
            :to="{ name: 'landing' }"
            flat
            rounded
          >
            Prodo
          </q-btn>
        </q-toolbar-title>

        <div>
          <ThemeButton />

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

    <!--
    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          Essential Links
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>
    -->

    <q-page-container
      class="q-pa-md"
    >
      <router-view />
    </q-page-container>

    <q-footer v-if="!$q.platform.is.mobile" class="q-pt-xl q-pb-lg q-px-md bg-white text-grey-5 flex flex-center">
      <div>
        @ {{ new Date().getFullYear() }} Martin Herich, version {{ version }}
      </div>
    </q-footer>
  </q-layout>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import EssentialLink, { EssentialLinkProps } from 'components/EssentialLink.vue'
import { useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'
import packageJson from '../../package.json'
import ThemeButton from 'components/toolkit/ThemeButton.vue'

const authStore = useAuthStore()

const onLogout = () => {
  authStore.logout()
    .then(() => {
      router.push({ name: 'landing' })
    })
}

const onLogin = async () => {
  router.push({ name: 'login' })
}

const isAuthenticated = computed(() => authStore.isAuthenticated)

const version = packageJson.version
</script>
