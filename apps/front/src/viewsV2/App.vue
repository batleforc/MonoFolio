<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';
import { onMounted } from 'vue';
import Header from '../component/home/Header.vue';
import NavBar from '../component/home/NavBar.vue';
import { usePageStore } from '../stores/page';

const indexStore = useIndexStore();
const pageStore = usePageStore();
onMounted(() => {
  if (!indexStore.isInitialized && !indexStore.homeLoading) {
    indexStore.init();
  }
});

</script>

<template>
  <Header />
  <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
  <RouterView v-if="indexStore.isInitialized" />
  <NavBar />
</template>