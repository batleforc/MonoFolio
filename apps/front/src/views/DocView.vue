<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useDocStore } from '../stores/doc';
import DocSidebar from '../component/Doc/DocSidebar.vue';
import { computed } from 'vue';

const route = useRoute();

let pathReactive = computed(() => {
    return Array.isArray(route.params.page) ? route.params.page : [route.params.page];
});


const docStore = useDocStore();
if (!docStore.inited && !docStore.docLoading) {
    docStore.init();
}

</script>

<template>
    <div id="title" class="docContainer" v-if="docStore.inited">
        <DocSidebar />
        <div class="docContent">
            <p>{{ pathReactive }}</p>
            <p>Doc content</p>
            <p v-for="truc in [...Array(100).keys()]" :key="truc">{{ truc }}</p>
            <footer>
                <p>Made with love and too much coffee @Batleforc - {{ new Date().getFullYear() }}</p>
            </footer>
        </div>
    </div>

</template>

<style lang="scss">
@import "../styles/DocView.scss";
</style>