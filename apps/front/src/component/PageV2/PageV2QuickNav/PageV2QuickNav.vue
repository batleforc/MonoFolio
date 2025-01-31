<script setup lang="ts">
import { DocHeader, Node } from '@portfolio/api-client';
import PageV2Heading from './PageV2Heading.vue';
import { computed } from 'vue';

const props = defineProps<{
  page: Node;
  metadata: DocHeader;
}>();

let heading = computed(() => {
  return props.page['children'].filter((node: Node) => node['Heading'] !== undefined);
});

</script>

<template>
  <div class="pageQuickNavWrapper" v-if="heading.length > 1">
    <div class="pageQuickNav">
      <div v-for="content in heading" :key="content">
        <PageV2Heading v-if="content['Heading'] !== undefined" :page="content" />
      </div>
    </div>
  </div>
</template>