<script setup lang="ts">

import { onMounted } from 'vue';
import { useIndexStore } from '../stores';

const indexStore = useIndexStore();

onMounted(() => {
  indexStore.setTitle("418 - I'm a teapot");
});

</script>

<template>
  <div class="relative flex size-full min-h-screen flex-col bg-[#221111] dark group/design-root overflow-x-hidden"
    style='font-family: "Space Grotesk", "Noto Sans", sans-serif;'>

    <h2 class="text-white text-[22px] font-bold leading-tight tracking-[-0.015em] px-4 pb-3 pt-5">I'm a teapot</h2>
    <p class="text-white text-base font-normal leading-normal pb-3 pt-1 px-4 text-center">
      The requested project is short and stout, here is my handle, here is my spout. I'm a little teapot
    </p>
    <div class="flex w-full grow bg-[#221111] @container p-4">
      <div class="w-full gap-1 overflow-hidden bg-[#221111] @[480px]:gap-2 aspect-[3/2] rounded-xl flex">
        <div class="w-full bg-center bg-no-repeat bg-cover aspect-auto rounded-none flex-1"
          style='background-image: url("https://cdn.usegalileo.ai/sdxl10/c2f6f229-69c5-49b4-bcaf-6dc4d0c61ba2.png");'>
        </div>
      </div>
    </div>
    <div class="flex px-4 py-3 justify-center">
      <RouterLink to="/"
        class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-xl h-10 px-4 flex-1 bg-[#78080e] text-white text-sm font-bold leading-normal tracking-[0.015em]">
        <span class="truncate">Take me home</span>
      </RouterLink>
    </div>
    <div class="h-5 bg-[#221111]"></div>
  </div>
</template>