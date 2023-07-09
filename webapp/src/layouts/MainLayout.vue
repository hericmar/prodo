<template>
  <q-layout view="lHh Lpr lFf">
    <q-header>
      <q-toolbar>
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
          <q-btn :to="{ name: 'index' }" flat>Prodo</q-btn>
        </q-toolbar-title>

        <div>
          <q-btn
            v-if="isAuthenticated"
            @click="onLogout" flat>{{ $t('logout') }}</q-btn>
          <q-btn
            v-else
            @click="onLogin" flat>{{ $t('login') }}</q-btn>
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

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-footer v-if="isAuthenticated" class="q-py-lg q-px-md bg-white text-grey-5 flex flex-center">
      <div>
        @ {{ new Date().getFullYear() }} Martin Herich, version {{ versionInfo.version }} (commit <a :href="`https://git.phire.cz/Phire/Prodo/commit/${getVersionInfo().revHash}`">{{ getVersionInfo().revHash }}</a>)
      </div>
    </q-footer>
  </q-layout>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import EssentialLink, { EssentialLinkProps } from 'components/EssentialLink.vue'
import { useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'
import versionInfo from 'src/assets/version.json'

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

const essentialLinks: EssentialLinkProps[] = [
  {
    title: 'Docs',
    caption: 'quasar.dev',
    icon: 'school',
    link: 'https://quasar.dev'
  },
  {
    title: 'Github',
    caption: 'github.com/quasarframework',
    icon: 'code',
    link: 'https://github.com/quasarframework'
  },
  {
    title: 'Discord Chat Channel',
    caption: 'chat.quasar.dev',
    icon: 'chat',
    link: 'https://chat.quasar.dev'
  },
  {
    title: 'Forum',
    caption: 'forum.quasar.dev',
    icon: 'record_voice_over',
    link: 'https://forum.quasar.dev'
  },
  {
    title: 'Twitter',
    caption: '@quasarframework',
    icon: 'rss_feed',
    link: 'https://twitter.quasar.dev'
  },
  {
    title: 'Facebook',
    caption: '@QuasarFramework',
    icon: 'public',
    link: 'https://facebook.quasar.dev'
  },
  {
    title: 'Quasar Awesome',
    caption: 'Community Quasar projects',
    icon: 'favorite',
    link: 'https://awesome.quasar.dev'
  }
]

/*
const leftDrawerOpen = ref(false)

function toggleLeftDrawer () {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
 */

const getVersionInfo = () => {
  return versionInfo
}
</script>
