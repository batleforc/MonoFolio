<script setup lang="ts">
import DocSidebar from '../component/Doc/DocSidebar.vue';
import Footer from '../component/helper/Footer.vue';
import WarnBan from '../component/helper/WarnBan.vue';
import PageRender from '../component/Page/PageRender.vue';
import PageV2Render from '../component/PageV2/PageV2Render.vue';
import { useDocStore } from '../stores/doc';
import { usePageStore } from '../stores/page';

const docStore = useDocStore();
const pageStore = usePageStore();
if (!docStore.inited && !docStore.docLoading) {
  docStore.init().then((data) => {
    if (data && data.has_index) {
      pageStore.fetchPage("index");
    }
  });
}
if (docStore.inited && docStore.docContent.has_index && pageStore.pageLoading === false && pageStore.pagePath !== "index") {
  pageStore.fetchPage("index");
}
</script>

<template>
  <div id="title" class="docContainer" v-if="docStore.inited">
    <DocSidebar />
    <div class="docContent">
      <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
      <div class="docRealContent" v-if="docStore.docContent.has_index && pageStore.page !== undefined">
        <PageRender :page="pageStore.page" />
      </div>
      <div class="docRealContent" v-if="docStore.docContent.has_index && pageStore.pageV2 !== undefined">
        <PageV2Render :page="pageStore.pageV2" />
      </div>
      <div v-if="!docStore.docContent.has_index">
        <h1>Doc home page</h1>
        <p>There is no index page for this documentation</p>
      </div>
      <Footer className="docFooter" />
    </div>
  </div>
</template>
