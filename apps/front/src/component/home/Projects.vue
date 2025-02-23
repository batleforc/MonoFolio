<script setup lang="ts">
import { useProjectStore } from '../../stores/project';
import IcoOrMedia from '../helper/IcoOrMedia.vue';
import { formatString } from '../../helper';
const projectStore = useProjectStore();

</script>

<template>
  <div id="projects" class="companies" v-if="projectStore.isInitialized">
    <h2>Projets</h2>
    <div class="grid">
      <RouterLink :to="{ name: 'projectcontent', params: { page: name } }"
        v-for="[name, project] in projectStore.getProjectContent" :key="name" class="card">
        <div class="company">
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
      </RouterLink>
    </div>
    <div class="flex justify-center">
      <RouterLink to="/projects" class="button-home">
        <span class="truncate">Plus de projets</span>
      </RouterLink>
    </div>
  </div>
</template>