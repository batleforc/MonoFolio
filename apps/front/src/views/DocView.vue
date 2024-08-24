<script setup lang="ts">
import { useDocStore } from '../stores/doc';

const docStore = useDocStore();
if (!docStore.inited && !docStore.docLoading) {
    docStore.init();
}

</script>

<template>
    <div id="title" class="container" v-if="docStore.inited">
        <h1>Doc sub page</h1>
        <p>{{ $route.params.page.join("/") }}</p>
        <div>
            <p v-for="sidebar in docStore.docContent.sub_categories" :key="sidebar.name">
                {{ sidebar.name }}
            </p>
        </div>
    </div>
</template>