<script setup lang="ts">

import { onMounted } from 'vue';
import { useIndexStore } from '../stores';

const indexStore = useIndexStore();

onMounted(() => {
  indexStore.setTitle("404 - Not Found");
});

</script>

<template>
  <div class="relative flex size-full min-h-screen flex-col bg-[#221111] dark group/design-root overflow-x-hidden"
    style='font-family: "Space Grotesk", "Noto Sans", sans-serif;'>
    <div class="@container">
      <div class="@[480px]:px-4 @[480px]:py-3">
        <div
          class="w-full bg-center bg-no-repeat bg-cover flex flex-col justify-end overflow-hidden bg-[#221111] @[480px]:rounded-xl min-h-[218px]"
          style='background-image: url("https://cdn.usegalileo.ai/sdxl10/4ac09907-8539-474d-b648-c0f63711b7ac.png");'>
        </div>
      </div>
    </div>
    <h1 class="text-white text-[22px] font-bold leading-tight tracking-[-0.015em] px-4 text-center pb-3 pt-5">Looks like
      you're lost in the network</h1>
    <p class="text-white text-base font-normal leading-normal pb-3 pt-1 px-4 text-center">
      The page you are looking for was not found. Please check the URL or go back to the homepage.
    </p>
    <div class="flex px-4 py-3 justify-center">
      <RouterLink to="/"
        class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-xl h-10 px-4 flex-1 bg-[#78080e] text-white text-sm font-bold leading-normal tracking-[0.015em]">
        <span class="truncate">Take me home</span>
      </RouterLink>
    </div>
    <div class="h-5 bg-[#221111]"></div>
  </div>
</template>
