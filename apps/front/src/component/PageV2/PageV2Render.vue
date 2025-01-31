<script setup lang="ts">
import { PageV2 } from '@portfolio/api-client';
import PageMetadata from './PageV2Metadata.vue';
import PageV2Content from './PageV2Content.vue';
import PageV2QuickNav from './PageV2QuickNav/PageV2QuickNav.vue';
defineProps<{
  page: PageV2;
}>();

</script>

<template>
  <div class="pageView">
    <PageMetadata v-if="page.metadata !== undefined" :metadata="page.metadata" />
    <div class="pageWrapper">
      <div class="pageContent my-2 grow" v-if="page.parsed_content !== undefined">
        <PageV2Content v-for="content in page.parsed_content['Root']['children']" :key="content" :page="content"
          :checked="undefined" />
      </div>
      <PageV2QuickNav v-if="page.parsed_content !== undefined && page.metadata !== undefined" :metadata="page.metadata"
        :page="page.parsed_content['Root']" />
    </div>
  </div>
</template>
