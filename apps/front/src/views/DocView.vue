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
    <div class="docTest">
        <div id="title" class="docContainer" v-if="docStore.inited">
            <DocSidebar />
            <div class="docContent">
                <p>{{ pathReactive }}</p>
                <p>Doc content</p>
            </div>
        </div>
    </div>
</template>

<style lang="scss">
.docTest {
    @apply flex grow;
}

.docContainer {
    @apply flex grow;
}

.docContent {
    @apply grow;
}
</style>