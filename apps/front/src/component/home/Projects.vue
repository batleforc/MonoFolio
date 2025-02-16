<script setup lang="ts">
import { useProjectStore } from '../../stores/project';
import IcoOrMedia from '../helper/IcoOrMedia.vue';

const projectStore = useProjectStore();

const formatString = (str: string) => {
  let target = str.substring(0, 35);
  if (target.length === 35) {
    target += '...';
  }
  return target;
};

</script>

<template>
  <div class="companies" v-if="projectStore.isInitialized">
    <h1>Projets</h1>
    <div class="grid">
      <div class="company" v-for="[name, project] in projectStore.getProjectContent" :key="name">
        <div v-if="project.metadata.image === undefined || project.metadata.image === null" class="image" style="
                background-image: url('https://cdn.usegalileo.ai/sdxl10/61a374be-15b4-4dbe-8685-7d5d8bba700a.png');
              "></div>
        <IcoOrMedia v-else :media="project.metadata.image" class-name="image" :alt="project.metadata.title" />
        <div>
          <p>{{ project.metadata.title }}</p>
          <p class="description">
            {{ formatString(project.metadata.description) }}
          </p>
        </div>
      </div>
    </div>
    <div class="flex justify-center">
      <RouterLink to="/projects" class="button-home">
        <span class="truncate">Plus de projets</span>
      </RouterLink>
    </div>
  </div>
</template>