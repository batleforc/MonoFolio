<script setup lang="ts">
import { useIndexStore } from '../../stores';
const indexStore = useIndexStore();
</script>

<template>
  <header>
    <h2>{{ indexStore.title }}</h2>
  </header>
</template>