<!-- app/layouts/default.vue -->
<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
const route = useRoute()

const routes = computed<NavigationMenuItem[]>(() => [
  { label: 'Accueil', to: '/', active: route.path === '/' },
  { label: 'About', to: '/about', active: route.path.startsWith('/about') },
])
</script>

<template>
  <NuxtHeader title="Games" to="/"
  :toggle="{
      color: 'primary',
      variant: 'subtle',
      class: 'rounded-full'
    }"
  >
    <!-- Centre (slot par défaut): ton menu principal -->
    <NuxtNavigationMenu :items="routes" />

    <!-- Zone droite -->
    <template #right>
      <NuxtColorModeButton />
    </template>
    <template #body>
      <NuxtNavigationMenu :items="routes" orientation="vertical" />
    </template>
  </NuxtHeader>

  <NuxtMain>
    <slot />
  </NuxtMain>

  <NuxtFooter>
    <template #left>
      <p class="text-muted text-sm">Copyright © {{ new Date().getFullYear() }}</p>
    </template>

    <template #right>
      <NuxtButton
        icon="i-simple-icons-discord"
        color="neutral"
        variant="ghost"
        to="https://go.nuxt.com/discord"
        target="_blank"
        aria-label="Discord"
      />
      <NuxtButton
        icon="i-simple-icons-x"
        color="neutral"
        variant="ghost"
        to="https://go.nuxt.com/x"
        target="_blank"
        aria-label="X"
      />
      <NuxtButton
        icon="i-simple-icons-github"
        color="neutral"
        variant="ghost"
        to="https://github.com/nuxt/nuxt"
        target="_blank"
        aria-label="GitHub"
      />
    </template>
  </NuxtFooter>
</template>
