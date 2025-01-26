<script setup lang="ts">
import { PageV2 } from '@portfolio/api-client';
import PageMetadata from './PageV2Metadata.vue';
import PageV2Content from './PageV2Content.vue';
defineProps<{
  page: PageV2;
}>();

</script>

<template>
  <div class="pageView">
    <PageMetadata v-if="page.metadata !== undefined" :metadata="page.metadata" />
    <div class="pageContent" v-if="page.parsed_content !== undefined">
      <PageV2Content v-for="content in page.parsed_content['Root']['children']" :key="content" :page="content" />
    </div>
  </div>
</template>
