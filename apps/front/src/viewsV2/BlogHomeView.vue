<script setup lang="ts">
import { onMounted } from 'vue';
import WarnBan from '../component/helper/WarnBan.vue';
import PageV2Adviser from '../component/PageV2/PageV2Adviser.vue';
import { useIndexStore } from '../stores';
import { useBlogStore } from '../stores/blog';
import { useDocStore } from '../stores/doc';
import { usePageStore } from '../stores/page';


const docStore = useDocStore();
const blogStore = useBlogStore();
const pageStore = usePageStore();
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
  <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
  <div class="flex flex-col"
    v-if="blogStore.isInitialized && !blogStore.blogLoading && blogStore.getBlogContent !== undefined">
    <PageV2Adviser v-for="post in blogStore.getBlogContent" :key="post[0]" :metadata="post[1].metadata"
      :default-collapsed="true" />
  </div>
</template>