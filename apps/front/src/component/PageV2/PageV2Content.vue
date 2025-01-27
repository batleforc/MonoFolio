<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Title from './PageV2NodeKind/PageV2Title.vue';
import PageV2Text from './PageV2NodeKind/PageV2Text.vue';

defineProps<{
  page: Node;
}>();
</script>

<template>
  <div class="pageContent">
    <PageV2Title v-if="page['Heading'] !== undefined" :value="page" :level="page['depth'] || 3" />
    <PageV2Text v-else-if="page['Text'] !== undefined" :value="page" />
    <p v-else>
      {{ page }}
    </p>
  </div>
</template>