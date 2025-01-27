<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <ul class="pageContent list-disc ml-4" v-if="value['List']['children'] !== undefined">
    <PageV2Content v-for="content in value['List']['children']" :page="content" :key="content" />
  </ul>
</template>
