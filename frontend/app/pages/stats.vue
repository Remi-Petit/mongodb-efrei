<script setup lang="ts">
// 1. Définition de la structure des données reçues
interface Stats {
  total_jeux: number;
  temps_total_heures: number;
  jeux_termines: number;
  score_moyen: number;
}

// 2. Récupération des données
const { data: stats, status, refresh } = await useFetch<Stats>('http://localhost:8080/api/stats', {
  server: false,
  lazy: true
})
</script>

<template>
  <NuxtContainer class="py-8 max-w-5xl mx-auto">
    
    <div class="flex items-center gap-4 mb-8">
      <NuxtButton to="/" icon="i-heroicons-arrow-left" variant="ghost" />
      <h1 class="text-3xl font-bold">Statistiques globales</h1>
    </div>

    <div v-if="status === 'pending'" class="flex justify-center py-20">
      <UIcon name="i-heroicons-arrow-path" class="w-8 h-8 animate-spin text-primary-500" />
    </div>

    <div v-else-if="status === 'error'" class="text-center py-10">
      <p class="text-red-500 mb-4">Impossible de charger les statistiques.</p>
      <NuxtButton @click="refresh()">Réessayer</NuxtButton>
    </div>

    <div v-else-if="stats" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">

      <NuxtCard class="text-center p-4 border-l-4 border-l-blue-500">
        <UIcon name="i-heroicons-archive-box" class="text-4xl text-blue-500 mb-2" />
        <p class="text-gray-500 text-sm font-medium uppercase tracking-wider">Total Jeux</p>
        <p class="text-4xl font-extrabold mt-2">{{ stats.total_jeux }}</p>
      </NuxtCard>

      <NuxtCard class="text-center p-4 border-l-4 border-l-green-500">
        <UIcon name="i-heroicons-clock" class="text-4xl text-green-500 mb-2" />
        <p class="text-gray-500 text-sm font-medium uppercase tracking-wider">Heures jouées</p>
        <p class="text-4xl font-extrabold mt-2">{{ stats.temps_total_heures }}h</p>
      </NuxtCard>

      <NuxtCard class="text-center p-4 border-l-4 border-l-purple-500">
        <UIcon name="i-heroicons-trophy" class="text-4xl text-purple-500 mb-2" />
        <p class="text-gray-500 text-sm font-medium uppercase tracking-wider">Jeux Terminés</p>
        <p class="text-4xl font-extrabold mt-2">{{ stats.jeux_termines }}</p>
        <p v-if="stats.total_jeux > 0" class="text-xs text-gray-400 mt-1">
          Soit {{ Math.round((stats.jeux_termines / stats.total_jeux) * 100) }}% du total
        </p>
      </NuxtCard>

      <NuxtCard class="text-center p-4 border-l-4 border-l-orange-500">
        <UIcon name="i-heroicons-star" class="text-4xl text-orange-500 mb-2" />
        <p class="text-gray-500 text-sm font-medium uppercase tracking-wider">Note Moyenne</p>
        <p class="text-4xl font-extrabold mt-2">{{ stats.score_moyen.toFixed(1) }}</p>
      </NuxtCard>

    </div>

  </NuxtContainer>
</template>