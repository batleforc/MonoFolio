<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';

const indexStore = useIndexStore();
if (!indexStore.isInitialized && !indexStore.homeLoading) {
  indexStore.init();
}

</script>

<template>
  <RouterView v-if="indexStore.isInitialized" />
</template>