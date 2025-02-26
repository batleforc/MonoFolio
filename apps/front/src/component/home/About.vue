<script setup lang="ts">
import { useIndexStore } from '../../stores';
const indexStore = useIndexStore();

</script>

<template>
  <h2 class="text-white text-[22px] font-bold leading-tight tracking-[-0.015em] px-4 pb-3 pt-5">
    A propos
  </h2>
  <div class="pb-3 pt-1 px-4">
    <p v-for="text in indexStore.homeContent.presentation.split('\n')" :key="text"
      class="text-white text-base font-normal leading-normal ">
      {{ text }}
    </p>
  </div>
</template>