<!-- app/layouts/default.vue -->
<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
const route = useRoute()

const routes = computed<NavigationMenuItem[]>(() => [
  { label: 'Accueil', to: '/', active: route.path === '/' },
  { label: 'About', to: '/about', active: route.path.startsWith('/about') },
])

const exportLoading = ref(false)

function downloadExport() {
  exportLoading.value = true;
  
  // On redirige vers l'URL de l'API. 
  // Comme le backend renvoie un fichier, le navigateur va lancer le téléchargement.
  window.location.href = 'http://localhost:8080/api/games/export';

  // On retire le chargement après 1 ou 2 secondes (juste pour l'effet visuel)
  setTimeout(() => {
    exportLoading.value = false;
  }, 2000);
}
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
      <NuxtButton color="primary" icon="i-heroicons-arrow-down-tray" @click="downloadExport">Export</NuxtButton>
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
