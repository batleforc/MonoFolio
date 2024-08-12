<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';
import Hero from '../component/Hero/Hero.vue';
import NavBar from '../component/NavBar/NavBar.vue';
console.log('Hello from front');

const indexStore = useIndexStore();
if (!indexStore.inited && !indexStore.homeLoading) {
  console.log('Init index store');
  indexStore.init();
}
</script>

<template>
  <Hero />
  <NavBar />
  <RouterView />
</template>
