<script setup lang="ts">
import { useIndexStore } from '../../stores';
import TextRotate from '../Hero/TextRotate.vue';
const indexStore = useIndexStore();
const smoothScroll = (event: MouseEvent) => {
  if (!event) return;
  event.preventDefault();
  document.querySelector(`#contact`)?.scrollIntoView({ behavior: 'smooth' });
}
</script>

<template>
  <div class="@container">
    <div class="@[480px]:p-4">
      <div id="hero" class="hero">
        <div class="flex flex-col gap-2 text-left">
          <h1>{{ indexStore.homeContent.shortDescription }}</h1>
          <h2>
            <TextRotate :texts="indexStore.homeContent.coverTitle" />
          </h2>
        </div>
        <button @click="smoothScroll">
          <span class="truncate">More</span>
        </button>
      </div>
    </div>
  </div>
</template>