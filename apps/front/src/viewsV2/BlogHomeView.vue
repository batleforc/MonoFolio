<script setup lang="ts">
import { onMounted } from 'vue';
import PageV2Adviser from '../component/PageV2/PageV2Adviser.vue';
import { useIndexStore } from '../stores';
import { useBlogStore } from '../stores/blog';
import { useDocStore } from '../stores/doc';


const docStore = useDocStore();
const blogStore = useBlogStore();
const indexStore = useIndexStore();



onMounted(() => {
  indexStore.setTitle("Blog  - Portfolio");
  if (!docStore.isInitialized && !docStore.docLoading) {
    docStore.init();
  }
  if (!blogStore.isInitialized && !blogStore.blogLoading) {
    blogStore.init();
  }
});

</script>

<template>
  <div class="blogListe">
    <div class="flex flex-col"
      v-if="blogStore.isInitialized && !blogStore.blogLoading && blogStore.getBlogContent !== undefined">
      <RouterLink v-for="post in blogStore.getBlogContent" :key="post[0]"
        :to="{ name: 'blogcontent', params: { page: post[0] } }">
        <PageV2Adviser :metadata="post[1].metadata" :default-collapsed="true" />
      </RouterLink>
    </div>
  </div>
</template>