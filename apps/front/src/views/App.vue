<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useIndexStore } from '../stores';
console.log('Hello from front');

const indexStore = useIndexStore();
if (!indexStore.inited && !indexStore.homeLoading) {
  console.log('Init index store');
  indexStore.init();
}
</script>

<template>
  <RouterView />
</template>
