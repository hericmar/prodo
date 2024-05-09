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
            :to="{ name: 'index' }" flat
          >
            Prodo
          </q-btn>
          <q-btn
            v-else
            :to="{ name: 'landing' }" flat
          >
            Prodo
          </q-btn>
        </q-toolbar-title>

        <div>
          <q-btn-dropdown
            v-if="isAuthenticated"
            :label="$t('profile')"
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
        @ {{ new Date().getFullYear() }} Martin Herich, version {{ version }}
      </div>
    </q-footer>
  </q-layout>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import EssentialLink, { EssentialLinkProps } from 'components/EssentialLink.vue'
import { useAuthStore } from 'stores/auth-store'
import { router } from 'src/router'
import packageJson from '../../package.json'

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

const version = packageJson.version

/*
const leftDrawerOpen = ref(false)

function toggleLeftDrawer () {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
 */
</script>
