<script setup lang="ts">
// 1. Interface
interface JeuVideo {
  _id?: any; // On met any pour accepter string ou { $oid: ... }
  titre: string;
  genre: string[];
  plateforme: string[];
  metacritic_score?: number;
  annee_sortie?: number;
}

// 2. Variables de sélection (Inputs utilisateur)
const search = ref('')
const selectedGenre = ref('')
const selectedPlatform = ref('')

// 3. Appel API : ON RÉCUPÈRE TOUT SANS FILTRE
// Note : J'ai retiré l'objet "query". On charge 100% de la base.
const { data: allGames, status, error, refresh } = await useFetch<JeuVideo[]>('http://localhost:8080/api/games', {
  lazy: true,
  server: false
});

// 4. Calcul des Options (Basé sur TOUS les jeux)
// Ces listes ne changeront pas même si tu filtres la vue
const genresOptions = computed(() => {
  if (!allGames.value) return []
  const uniqueGenres = [...new Set(allGames.value.flatMap(g => g.genre))].sort()
  return uniqueGenres.map(g => ({ label: g, value: g }))
})

const platformOptions = computed(() => {
  if (!allGames.value) return []
  const uniquePlatforms = [...new Set(allGames.value.flatMap(g => g.plateforme))].sort()
  return uniquePlatforms.map(p => ({ label: p, value: p }))
})

// 5. LE COEUR DU SYSTÈME : La liste filtrée (Computed)
// C'est cette variable qu'on va afficher dans le template
const filteredGames = computed(() => {
  if (!allGames.value) return []

  return allGames.value.filter(game => {
    // A. Filtre Titre (Insensible à la casse)
    const matchTitle = !search.value || game.titre.toLowerCase().includes(search.value.toLowerCase())

    // B. Filtre Genre (Vérifie si le genre sélectionné est PRÉSENT dans le tableau des genres du jeu)
    const matchGenre = !selectedGenre.value || game.genre.includes(selectedGenre.value)

    // C. Filtre Plateforme (Idem)
    const matchPlatform = !selectedPlatform.value || game.plateforme.includes(selectedPlatform.value)

    // On garde le jeu seulement si les 3 conditions sont vraies
    return matchTitle && matchGenre && matchPlatform
  })
})
</script>

<template>
  <div>
    <NuxtContainer class="py-8">
      
      <div class="flex flex-col md:flex-row justify-between items-end md:items-center gap-4 mb-8">
        <div>
          <h2 class="text-3xl font-bold mb-1">Ma Ludothèque</h2>
          <p class="text-gray-500 dark:text-gray-400 text-sm">
            <span v-if="allGames">
              {{ filteredGames.length }} / {{ allGames.length }} jeux affichés
            </span>
            <span v-else>Chargement...</span>
          </p>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-2 w-full md:w-auto">
          
          <NuxtSelect 
            v-model="selectedGenre"
            :items="genresOptions"
            placeholder="Genre"
            icon="i-heroicons-funnel"
            class="w-full sm:w-40"
          >
            <template #leading>
              <UIcon name="i-heroicons-funnel" v-if="!selectedGenre" class="text-gray-500" />
              <UIcon name="i-heroicons-x-mark" class="cursor-pointer text-gray-400 hover:text-red-500" v-else @click.stop="selectedGenre = ''" />
            </template>
          </NuxtSelect>

          <NuxtSelect 
            v-model="selectedPlatform"
            :items="platformOptions"
            placeholder="Plateforme"
            icon="i-heroicons-computer-desktop"
            class="w-full sm:w-40"
          >
             <template #leading>
              <UIcon name="i-heroicons-computer-desktop" v-if="!selectedPlatform" class="text-gray-500" />
              <UIcon name="i-heroicons-x-mark" class="cursor-pointer text-gray-400 hover:text-red-500" v-else @click.stop="selectedPlatform = ''" />
            </template>
          </NuxtSelect>

          <NuxtInput 
            v-model="search"
            icon="i-heroicons-magnifying-glass-20-solid"
            color="white"
            trailing
            placeholder="Rechercher..."
            class="w-full sm:w-60"
          />
        </div>
      </div>

      <div v-if="status === 'pending'" class="flex flex-col items-center justify-center py-20">
        <NuxtIcon name="i-heroicons-arrow-path" class="w-10 h-10 animate-spin text-primary-500 mb-4" />
        <p class="text-gray-500">Récupération des données...</p>
      </div>

      <div v-else-if="status === 'error'" class="bg-red-50 dark:bg-red-900/20 p-6 rounded-lg text-center border border-red-200 dark:border-red-800">
        <h3 class="text-lg font-bold text-red-600 dark:text-red-400">Erreur</h3>
        <p class="text-sm mb-4">Impossible de charger les jeux.</p>
        <NuxtButton color="red" variant="soft" @click="refresh()">Réessayer</NuxtButton>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        
        <NuxtCard v-for="game in filteredGames" :key="JSON.stringify(game._id) || game.titre" class="hover:ring-2 hover:ring-primary-500 transition-all cursor-pointer group">
          
          <template #header>
            <div class="flex justify-between items-start">
              <NuxtBadge v-if="game.metacritic_score" :color="game.metacritic_score > 90 ? 'green' : 'orange'">
                {{ game.metacritic_score }}
              </NuxtBadge>
              <span class="text-xs text-gray-500 font-mono">{{ game.annee_sortie }}</span>
            </div>
          </template>

          <h3 class="text-xl font-bold mb-2 group-hover:text-primary-500 transition-colors">{{ game.titre }}</h3>
          
          <div class="flex flex-wrap gap-1 mb-4">
            <NuxtBadge v-for="g in game.genre" :key="g" color="gray" variant="subtle" size="xs">
              {{ g }}
            </NuxtBadge>
          </div>

          <template #footer>
            <div class="flex justify-between items-center h-6">
              <div class="text-xs text-gray-500 truncate w-3/4">
                {{ game.plateforme ? game.plateforme.join(', ') : '' }}
              </div>
              <NuxtButton icon="i-heroicons-heart" variant="ghost" color="gray" />
            </div>
          </template>
        </NuxtCard>
      </div>
      
      <div v-if="status === 'success' && filteredGames.length === 0" class="text-center py-20 text-gray-500">
        <UIcon name="i-heroicons-face-frown" class="text-4xl mb-2 text-gray-400" />
        <p v-if="allGames && allGames.length > 0">Aucun résultat pour ces filtres.</p>
        <p v-else>La base de données est vide.</p>
        
        <NuxtButton 
          v-if="search || selectedGenre || selectedPlatform" 
          variant="link" 
          color="primary" 
          @click="search = ''; selectedGenre = ''; selectedPlatform = ''"
        >
          Réinitialiser les filtres
        </NuxtButton>
      </div>

    </NuxtContainer>
  </div>
</template>