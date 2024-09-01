<script setup lang="ts">
import { Page } from '@portfolio/api-client';
import PageMetadata from './PageMetadata.vue';
import PageContent from './PageContent.vue';
import { TitleBlock, transformContent } from '../../markdown';
import { ref, watch } from 'vue';
const props = defineProps<{
    page: Page;
}>();

let contentBlock = ref<TitleBlock[]>(transformContent(props.page.content));

watch(() => props.page.content, (newValue, oldValue) => {
    if (newValue !== oldValue) {
        contentBlock.value = transformContent(newValue);
    }
})

</script>

<template>
    <div class="pageView">
        <PageMetadata :metadata="page.metadata" />
        <div class="pageContent">
            <PageContent v-for="block in contentBlock" :key="block.value" :content="block" />
        </div>
    </div>
</template>