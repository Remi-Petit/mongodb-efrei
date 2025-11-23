<script setup lang="ts">
const route = useRoute()
const router = useRouter()
const gameId = route.params.id as string

// 1. Chargement des données du jeu
const { data: game, status, refresh } = await useFetch<any>(`http://localhost:8080/api/games/${gameId}`)

// 2. Variables tampons pour le formulaire
const genreInput = ref('')
const platformInput = ref('')
const loading = ref(false)

const state = reactive({
  titre: '',
  editeur: '',
  developpeur: '',
  annee_sortie: undefined as number | undefined,
  metacritic_score: undefined as number | undefined,
  temps_jeu_heures: undefined as number | undefined,
  termine: false,
  favori: false // On garde le favori en mémoire
})

// 3. Initialisation du formulaire quand les données arrivent
// On utilise watch car "game" peut arriver avec un léger délai
watch(game, (newGame) => {
  if (newGame) {
    state.titre = newGame.titre
    state.editeur = newGame.editeur || ''
    state.developpeur = newGame.developpeur || ''
    state.annee_sortie = newGame.annee_sortie
    state.metacritic_score = newGame.metacritic_score
    state.temps_jeu_heures = newGame.temps_jeu_heures
    state.termine = newGame.termine
    state.favori = newGame.favori

    // Transformation Array -> String pour l'affichage (avec virgules)
    genreInput.value = newGame.genre ? newGame.genre.join(', ') : ''
    platformInput.value = newGame.plateforme ? newGame.plateforme.join(', ') : ''
  }
}, { immediate: true })

// 4. Validation
const isFormValid = computed(() => {
  return state.titre.trim().length > 0 && 
         genreInput.value.trim().length > 0 && 
         platformInput.value.trim().length > 0
})

// 5. Mise à jour (UPDATE / PUT)
async function updateGame() {
  if (!isFormValid.value) return
  loading.value = true

  // Transformation String -> Array
  const genresArray = genreInput.value.split(',').map(s => s.trim()).filter(s => s.length > 0)
  const platformsArray = platformInput.value.split(',').map(s => s.trim()).filter(s => s.length > 0)

  const payload = {
    ...state,
    genre: genresArray,
    plateforme: platformsArray,
    // On met à jour la date de modification
    date_modification: new Date().toISOString(),
    // On garde la date d'ajout originale si elle existe, sinon maintenant
    date_ajout: game.value?.date_ajout || new Date().toISOString()
  }

  try {
    await $fetch(`http://localhost:8080/api/games/${gameId}`, {
      method: 'PUT',
      body: payload
    })
    // Retour à l'accueil
    router.push('/')
  } catch (err) {
    console.error(err)
    alert("Erreur lors de la modification.")
  } finally {
    loading.value = false
  }
}

// 6. Suppression (DELETE)
async function deleteGame() {
  if (!confirm("Êtes-vous sûr de vouloir supprimer ce jeu définitivement ?")) return
  
  try {
    await $fetch(`http://localhost:8080/api/games/${gameId}`, {
      method: 'DELETE'
    })
    router.push('/')
  } catch (err) {
    console.error(err)
    alert("Impossible de supprimer le jeu.")
  }
}
</script>

<template>
  <NuxtContainer class="py-8 max-w-2xl mx-auto">
    
    <div class="flex items-center justify-between mb-8">
      <div class="flex items-center gap-4">
        <NuxtButton to="/" icon="i-heroicons-arrow-left" variant="ghost" />
        <h1 class="text-2xl font-bold" v-if="game">Modifier "{{ game.titre }}"</h1>
        <h1 class="text-2xl font-bold" v-else>Chargement...</h1>
      </div>
      
      <NuxtButton 
        v-if="game"
        color="red" 
        variant="ghost" 
        icon="i-heroicons-trash" 
        @click="deleteGame"
      >
        Supprimer
      </NuxtButton>
    </div>

    <div v-if="status === 'pending'" class="flex justify-center py-10">
      <UIcon name="i-heroicons-arrow-path" class="animate-spin text-3xl text-primary-500" />
    </div>

    <form v-else-if="game" @submit.prevent="updateGame" class="space-y-6">

      <div class="flex flex-col gap-2">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
          Titre du jeu <span class="text-red-500">*</span>
        </label>
        <NuxtInput v-model="state.titre" />
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
            Genres <span class="text-red-500">*</span>
          </label>
          <NuxtInput v-model="genreInput" />
          <p class="text-xs text-gray-500">Séparés par une virgule</p>
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
            Plateformes <span class="text-red-500">*</span>
          </label>
          <NuxtInput v-model="platformInput" />
          <p class="text-xs text-gray-500">Séparées par une virgule</p>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Éditeur</label>
          <NuxtInput v-model="state.editeur" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Développeur</label>
          <NuxtInput v-model="state.developpeur" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Année</label>
          <NuxtInput type="number" v-model.number="state.annee_sortie" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Metacritic</label>
          <NuxtInput type="number" v-model.number="state.metacritic_score" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Temps (h)</label>
          <NuxtInput type="number" step="0.5" v-model.number="state.temps_jeu_heures" />
        </div>
      </div>

      <div class="p-4 bg-gray-50 dark:bg-gray-800/50 rounded-lg border border-gray-200 dark:border-gray-700">
        <NuxtCheckbox 
          v-model="state.termine" 
          label="Jeu terminé ?" 
        />
      </div>

      <div class="flex justify-between pt-4 border-t border-gray-200 dark:border-gray-700">
        <NuxtButton 
          type="button" 
          color="red" 
          variant="soft" 
          icon="i-heroicons-trash"
          @click="deleteGame"
        >
          Supprimer ce jeu
        </NuxtButton>

        <div class="flex gap-3">
          <NuxtButton to="/" variant="ghost">Annuler</NuxtButton>
          <NuxtButton type="submit" :loading="loading" :disabled="!isFormValid">
            Sauvegarder les modifications
          </NuxtButton>
        </div>
      </div>

    </form>
    
    <div v-else class="text-center py-10">
        <h2 class="text-xl font-bold text-red-500">Jeu introuvable</h2>
        <NuxtButton to="/" class="mt-4">Retour à l'accueil</NuxtButton>
    </div>

  </NuxtContainer>
</template>