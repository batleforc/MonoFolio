<script setup lang="ts">
import { DocCategory } from '@portfolio/api-client';
import { computed, ref } from 'vue';
import IcoMoonSVG from '../helper/IcoMoonSVG.vue';

const props = defineProps<{
  docContent: DocCategory;
  path: Array<string>;
}>();

const pageFiltered = computed(() => {
  return props.docContent.pages.filter(page => page.name !== 'index');
});

const hasContent = computed(() => {
  return pageFiltered.value.length > 0 || props.docContent.sub_categories.length > 0;
});

const fromOpen = computed(() => {
  return props.path.length > 0 && props.docContent.name === props.path[0];
});

const pathToChild = computed(() => {
  if (fromOpen.value) {
    return props.path.slice(1);
  }
  return [];
});

const pathIndex = computed(() => {
  if (!props.docContent.has_index) {
    return [];
  }
  return props.docContent.pages.find(page => page.name === 'index').path.split('/').filter(path => path !== 'index');
});

const sidebarOpen = ref(fromOpen.value);

</script>

<template>
  <div class="docSidebarItem">
    <p @click="sidebarOpen = !sidebarOpen" class="docSidebarTitle docSidebarContentItem">
      <template v-if="!props.docContent.has_index">
        {{ props.docContent.name }}
      </template>
      <RouterLink v-else :to="{ name: 'doccontent', params: { page: pathIndex } }">
        {{ props.docContent.name }}
      </RouterLink>

      <IcoMoonSVG v-if="hasContent" :class="sidebarOpen ? 'docSidebarOpen' : ''" class="docSidebarTitleChevron"
        icon="chevron-down" />
    </p>
    <div :class="sidebarOpen ? 'docSidebarOpen' : ''" class="docSidebarContent">
      <RouterLink class="docSidebarContentItem" v-for="page in pageFiltered" :key="page.name"
        :to="{ name: 'doccontent', params: { page: page.path.split('/') } }">
        {{ page.metadata.title }}
      </RouterLink>
      <div v-if="props.docContent.sub_categories.length > 0" class="docSidebarSubItem">
        <DocSidebarItem :path="pathToChild" v-for="categorie in props.docContent.sub_categories"
          v-bind:key="categorie.name" :docContent="categorie" />
      </div>
    </div>

  </div>
</template>
