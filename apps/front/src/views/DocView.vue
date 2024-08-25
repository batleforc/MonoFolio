<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useDocStore } from '../stores/doc';

const route = useRoute();

let path = Array.isArray(route.params.page) ? route.params.page.join("/") : route.params.page;


const docStore = useDocStore();
if (!docStore.inited && !docStore.docLoading) {
    docStore.init();
}

</script>

<template>
    <div id="title" class="docContainer" v-if="docStore.inited">
        <div class="docSidebar">
            <p>Doc Sidebar</p>
            <p v-for="sidebar in docStore.docContent.sub_categories" :key="sidebar.name">
                {{ sidebar.name }}
            </p>
        </div>
        <div class="docContent">
            <p>{{ path }}</p>
            <p>Doc content</p>
        </div>
    </div>
</template>

<style lang="scss">
.docContainer {
    @apply flex;
}

.docContent {
    @apply grow;
}
</style>