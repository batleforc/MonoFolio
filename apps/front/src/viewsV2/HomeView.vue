<script setup lang="ts">

import { onMounted } from 'vue';
import Hero from '../component/home/Hero.vue';
import Milestones from '../component/home/Milestones.vue';
import { useIndexStore } from '../stores';
import { useProjectStore } from '../stores/project';
import Projects from '../component/home/Projects.vue';

const indexStore = useIndexStore();
const projectStore = useProjectStore();

onMounted(() => {
  if (!projectStore.isInitialized)
    projectStore.init();
  indexStore.setTitle(`${indexStore.homeContent.name} - Portfolio`);
});

</script>

<template>
  <Hero />
  <Milestones />
  <Projects />
  <div class="contact">
    <div class="item">
      <p class="label">Location</p>
      <p>France, Somewhere or Nowhere</p>
    </div>
    <div class="item">
      <p class="label">Availability</p>
      <p>Working at Macif</p>
    </div>
    <div class="item full">
      <p class="label">Email</p>
      <p>maxime.lerichepro@gmail.com</p>
    </div>
  </div>
</template>