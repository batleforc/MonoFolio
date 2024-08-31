<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useDocStore } from '../../stores/doc';
import DocSidebarItem from './DocSidebarItem.vue';

const route = useRoute();
const docStore = useDocStore();

let path = Array.isArray(route.params.page) ? route.params.page : [route.params.page];

</script>

<template>
    <aside class="docSidebar">
        <RouterLink :to="{ name: 'doc' }">Doc Home</RouterLink>
        <DocSidebarItem :path="path" v-for="categorie in docStore.docContent.sub_categories" v-bind:key="categorie.name"
            :docContent="categorie" />
    </aside>
</template>

<style lang="scss">
@import "../../var.scss";

.docSidebar {
    @apply border-r-2 min-w-48 px-4 py-2 hidden md:flex flex-col sticky;
    background-color: $color-bgCover;
    color: $color-textCover;
}
</style>