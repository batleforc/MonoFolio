<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';
import Hero from '../component/Hero/Hero.vue';
import NavBar from '../component/NavBar/NavBar.vue';

const indexStore = useIndexStore();
if (!indexStore.isInitialized && !indexStore.homeLoading) {
  indexStore.init();
}
</script>

<template>
  <Hero v-if="$route.meta.hero" />
  <div class="flex flex-col min-h-full">
    <NavBar v-if="indexStore.isInitialized" :link="indexStore.homeContent.url" />
    <RouterView />
  </div>

</template>
