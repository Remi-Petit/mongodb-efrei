<script setup lang="ts">
const router = useRouter()

// 1. Variables tampons (Inputs texte)
const genreInput = ref('')
const platformInput = ref('')

// 2. État du formulaire
const state = reactive({
  titre: '',
  editeur: '',
  developpeur: '',
  annee_sortie: undefined as number | undefined,
  metacritic_score: undefined as number | undefined,
  temps_jeu_heures: undefined as number | undefined,
  termine: false
})

const loading = ref(false)

// 3. Validation
const isFormValid = computed(() => {
  return state.titre.trim().length > 0 && 
         genreInput.value.trim().length > 0 && 
         platformInput.value.trim().length > 0
})

// 4. Envoi
async function submitGame() {
  if (!isFormValid.value) return

  loading.value = true
  
  // Transformation String -> Array
  const genresArray = genreInput.value.split(',').map(s => s.trim()).filter(s => s.length > 0)
  const platformsArray = platformInput.value.split(',').map(s => s.trim()).filter(s => s.length > 0)

  const payload = {
    ...state,
    genre: genresArray,
    plateforme: platformsArray,
    annee_sortie: state.annee_sortie || null,
    metacritic_score: state.metacritic_score || null,
    temps_jeu_heures: state.temps_jeu_heures || null,
    date_ajout: new Date().toISOString(),
    date_modification: new Date().toISOString(),
    favori: false
  }

  try {
    await $fetch('http://localhost:8080/api/games', {
      method: 'POST',
      body: payload
    })
    router.push('/')
  } catch (err) {
    console.error(err)
    alert("Erreur lors de la création.")
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <NuxtContainer class="py-8 max-w-2xl mx-auto">
    
    <div class="flex items-center gap-4 mb-8">
      <NuxtButton to="/" icon="i-heroicons-arrow-left" variant="ghost" />
      <h1 class="text-2xl font-bold">Ajouter un jeu</h1>
    </div>

    <form @submit.prevent="submitGame" class="space-y-6">

      <div class="flex flex-col gap-2">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
          Titre du jeu <span class="text-red-500">*</span>
        </label>
        <NuxtInput v-model="state.titre" placeholder="Ex: Elden Ring" autofocus />
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
            Genres <span class="text-red-500">*</span>
          </label>
          <NuxtInput v-model="genreInput" placeholder="Ex: RPG, Action" />
          <p class="text-xs text-gray-500">Séparez les genres par une virgule</p>
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">
            Plateformes <span class="text-red-500">*</span>
          </label>
          <NuxtInput v-model="platformInput" placeholder="Ex: PC, PS5" />
          <p class="text-xs text-gray-500">Séparez les plateformes par une virgule</p>
        </div>

      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Éditeur</label>
          <NuxtInput v-model="state.editeur" placeholder="Ex: FromSoftware" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Développeur</label>
          <NuxtInput v-model="state.developpeur" placeholder="Ex: FromSoftware" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Année</label>
          <NuxtInput type="number" v-model.number="state.annee_sortie" placeholder="2022" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Metacritic</label>
          <NuxtInput type="number" v-model.number="state.metacritic_score" placeholder="0-100" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-200">Temps (h)</label>
          <NuxtInput type="number" step="0.5" v-model.number="state.temps_jeu_heures" placeholder="Heures" />
        </div>
      </div>

      <div class="p-4 bg-gray-50 dark:bg-gray-800/50 rounded-lg border border-gray-200 dark:border-gray-700">
        <NuxtCheckbox 
          v-model="state.termine" 
          label="Jeu terminé ?" 
          help="Cocher si vous avez fini l'histoire principale"
        />
      </div>

      <div class="flex justify-end gap-3 pt-4">
        <NuxtButton to="/" variant="ghost">Annuler</NuxtButton>
        <NuxtButton type="submit" :loading="loading" :disabled="!isFormValid">
          Enregistrer
        </NuxtButton>
      </div>

    </form>
  </NuxtContainer>
</template>