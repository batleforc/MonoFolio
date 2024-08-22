<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';
import Hero from '../component/Hero/Hero.vue';
import NavBar from '../component/NavBar/NavBar.vue';

const indexStore = useIndexStore();
if (!indexStore.inited && !indexStore.homeLoading) {
  indexStore.init();
}
</script>

<template>
  <Hero />
  <NavBar v-if="indexStore.inited" :link="indexStore.homeContent.url" />
  <RouterView />
</template>
