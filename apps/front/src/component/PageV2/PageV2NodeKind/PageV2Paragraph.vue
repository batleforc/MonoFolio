<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <div class="pageContent items-center" v-if="value['Paragraph']['children'] !== undefined">
    <PageV2Content v-for="content in value['Paragraph']['children']" :page="content" :key="content" />
  </div>
</template>
