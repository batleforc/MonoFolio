<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useBlogStore } from '../stores/blog';
import { useDocStore } from '../stores/doc';
import { usePageStore } from '../stores/page';
import { computed, watch } from 'vue';
import PageRender from '../component/Page/PageRender.vue';
import PageV2Render from '../component/PageV2/PageV2Render.vue';
import WarnBan from '../component/helper/WarnBan.vue';
import Footer from '../component/helper/Footer.vue';

const route = useRoute();
const docStore = useDocStore();
const blogStore = useBlogStore();
const pageStore = usePageStore();

let pathReactive = computed(() => {
  return (Array.isArray(route.params.page) ? route.params.page.join('') : route.params.page);
});

if (!docStore.inited && !docStore.docLoading) {
  docStore.init();
}
if (!blogStore.inited && !blogStore.blogLoading) {
  blogStore.init().then(() => {
    if (blogStore.getBlogContentUnordered[pathReactive.value] === undefined) {
      return;
    }
    pageStore.fetchPage(blogStore.getBlogContentUnordered[pathReactive.value].path);
    console.log('blogStore initialized');
  });
} else if (pageStore.pageLoading === false && pageStore.pagePath !== blogStore.getBlogContentUnordered[pathReactive.value].path) {
  pageStore.fetchPage(blogStore.getBlogContentUnordered[pathReactive.value].path);
}

watch(pathReactive, (newVal) => {
  if (pageStore.pageLoading === false && pageStore.pagePath !== newVal) {
    pageStore.fetchPage(blogStore.getBlogContentUnordered[newVal].path);
  }
});

</script>

<template>
  <div id="title" class="docContainer" v-if="docStore.inited && blogStore.inited">
    <div class="docContent">
      <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
      <div class="docRealContent" v-if="pageStore.page !== undefined">
        <PageRender :page="pageStore.page" />
      </div>
      <div class="docRealContent" v-if="pageStore.pageV2 !== undefined">
        <PageV2Render :page="pageStore.pageV2" />
      </div>
      <div v-if="!pageStore.pageLoading && pageStore.page === undefined && pageStore.pageV2 === undefined">
        <h1>Blog page</h1>
        <p>There is no index page for this documentation</p>
      </div>
      <Footer className="docFooter" />
    </div>
  </div>
</template>