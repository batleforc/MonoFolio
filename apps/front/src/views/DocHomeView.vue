<script setup lang="ts">
import DocSidebar from '../component/Doc/DocSidebar.vue';
import PageRender from '../component/Page/PageRender.vue';
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
            <div class="docRealContent" v-if="docStore.docContent.has_index && pageStore.page !== undefined">
                <PageRender :page="pageStore.page" />
            </div>
            <div v-if="!docStore.docContent.has_index">
                <h1>Doc home page</h1>
                <p>There is no index page for this documentation</p>
            </div>
            <footer class="docFooter">
                <p>Made with love and too much coffee @Batleforc - {{ new Date().getFullYear() }}</p>
            </footer>
        </div>
    </div>
</template>

<style lang="scss">
@import "../styles/DocView.scss";
</style>