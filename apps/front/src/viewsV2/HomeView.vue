<script setup lang="ts">

import { onMounted } from 'vue';
import Hero from '../component/home/Hero.vue';
import Milestones from '../component/home/Milestones.vue';
import { useIndexStore } from '../stores';
import { useProjectStore } from '../stores/project';
import Projects from '../component/home/Projects.vue';
import Contact from '../component/home/Contact.vue';

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
  <Contact />
</template>