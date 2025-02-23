<script setup lang="ts">
import { onMounted } from 'vue';
import PageV2Adviser from '../component/PageV2/PageV2Adviser.vue';
import { useIndexStore } from '../stores';
import { useDocStore } from '../stores/doc';
import { useProjectStore } from '../stores/project';


const docStore = useDocStore();
const projectStore = useProjectStore();
const indexStore = useIndexStore();



onMounted(() => {
  indexStore.setTitle("Project  - Portfolio");
  if (!docStore.isInitialized && !docStore.docLoading) {
    docStore.init();
  }
  if (!projectStore.isInitialized && !projectStore.projectLoading) {
    projectStore.init();
  }
});

</script>

<template>
  <div class="blogListe">
    <div class="flex flex-col"
      v-if="projectStore.isInitialized && !projectStore.projectLoading && projectStore.getProjectContent !== undefined">
      <RouterLink v-for="post in projectStore.getProjectContent" :key="post[0]"
        :to="{ name: 'projectcontent', params: { page: post[0] } }">
        <PageV2Adviser :metadata="post[1].metadata" :default-collapsed="true" />
      </RouterLink>
    </div>
  </div>
</template>