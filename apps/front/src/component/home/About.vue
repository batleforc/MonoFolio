<script setup lang="ts">
import { useIndexStore } from '../../stores';
const indexStore = useIndexStore();

</script>

<template>
  <h2 class="text-white text-[22px] font-bold leading-tight tracking-[-0.015em] px-4 pb-3 pt-5">
    A propos
  </h2>
  <p class="text-white text-base font-normal leading-normal pb-3 pt-1 px-4">
    {{ indexStore.homeContent.presentation }}
  </p>
</template>