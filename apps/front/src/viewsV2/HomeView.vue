<script setup lang="ts">

import { onMounted } from 'vue';
import Hero from '../component/home/Hero.vue';
import Milestones from '../component/home/Milestones.vue';
import { useIndexStore } from '../stores';
import { useProjectStore } from '../stores/project';
import Projects from '../component/home/Projects.vue';
import Contact from '../component/home/Contact.vue';
import { useRoute } from 'vue-router';
import About from '../component/home/About.vue';

const indexStore = useIndexStore();
const projectStore = useProjectStore();
const route = useRoute();

onMounted(() => {
  if (!projectStore.isInitialized)
    projectStore.init();
  indexStore.setTitle(`${indexStore.homeContent.name} - Portfolio`);
  if (route.hash.length > 0) {
    const el = document.getElementById(route.hash.slice(1));
    if (el) {
      el.scrollIntoView({ behavior: 'smooth' });
    }
  }
});

</script>

<template>
  <Hero />
  <About />
  <Milestones />
  <Projects />
  <Contact />
</template>