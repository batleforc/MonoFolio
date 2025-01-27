<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Title from './PageV2NodeKind/PageV2Title.vue';
import PageV2Text from './PageV2NodeKind/PageV2Text.vue';
import PageV2Paragraph from './PageV2NodeKind/PageV2Paragraph.vue';
import PageV2Code from './PageV2NodeKind/PageV2Code.vue';

defineProps<{
  page: Node;
}>();
</script>

<template>
  <div class="pageContent">
    <PageV2Title v-if="page['Heading'] !== undefined" :value="page" :level="page['depth'] || 3" />
    <PageV2Text v-else-if="page['Text'] !== undefined" :value="page" />
    <PageV2Paragraph v-else-if="page['Paragraph'] !== undefined" :value="page" />
    <PageV2Code v-else-if="page['Code'] !== undefined" :value="page" />
    <p v-else>
      {{ page }}
    </p>
  </div>
</template>