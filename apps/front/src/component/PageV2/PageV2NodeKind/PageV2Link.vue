<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <a :href="value['Link']['url']" class="pageContent list-disc italic font-medium"
    v-if="value['Link']['children'] !== undefined">
    <PageV2Content v-for="content in value['Link']['children']" :page="content" :key="content" />
  </a>
</template>
