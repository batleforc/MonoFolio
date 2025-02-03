<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useDocStore } from '../stores/doc';
import DocSidebar from '../component/Doc/DocSidebar.vue';
import { computed, watch } from 'vue';
import { usePageStore } from '../stores/page';
import PageRender from '../component/Page/PageRender.vue';
import PageV2Render from '../component/PageV2/PageV2Render.vue';
import Footer from '../component/helper/Footer.vue';
import WarnBan from '../component/helper/WarnBan.vue';

const route = useRoute();

let pathReactive = computed(() => {
  return (Array.isArray(route.params.page) ? route.params.page : [route.params.page]).join('/');
});


const docStore = useDocStore();
const pageStore = usePageStore();
if (!docStore.isInitialized && !docStore.docLoading) {
  docStore.init().then(() => {
    pageStore.fetchPage(pathReactive.value);
  });
}
else if (pageStore.pageLoading === false && pageStore.pagePath !== pathReactive.value) {
  pageStore.fetchPage(pathReactive.value);
}

watch(pathReactive, (newVal) => {
  if (pageStore.pageLoading === false && pageStore.pagePath !== newVal) {
    pageStore.fetchPage(newVal);
  }
});
</script>

<template>
  <div id="title" class="docContainer" v-if="docStore.isInitialized">

    <DocSidebar />
    <div class="docContent">
      <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
      <div class="docRealContent" v-if="pageStore.page !== undefined">
        <PageRender :page="pageStore.page" />
      </div>
      <div class="docRealContent" v-if="pageStore.pageV2 !== undefined">
        <PageV2Render :page="pageStore.pageV2" />
      </div>
      <div v-if="!pageStore.pageLoading && pageStore.page === undefined && pageStore.pageV2 === undefined">
        <h1>Doc home page</h1>
        <p>There is no index page for this documentation</p>
      </div>
      <Footer className="docFooter" />
    </div>
  </div>

</template>
