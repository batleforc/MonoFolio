<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Title from './PageV2NodeKind/PageV2Title.vue';
import PageV2Text from './PageV2NodeKind/PageV2Text.vue';
import PageV2Paragraph from './PageV2NodeKind/PageV2Paragraph.vue';
import PageV2Code from './PageV2NodeKind/PageV2Code.vue';
import PageV2List from './PageV2NodeKind/PageV2List.vue';
import PageV2ListItem from './PageV2NodeKind/PageV2ListItem.vue';
import PageV2InlineCode from './PageV2NodeKind/PageV2InlineCode.vue';
import PageV2Link from './PageV2NodeKind/PageV2Link.vue';
import PageV2Strong from './PageV2NodeKind/PageV2Strong.vue';
import PageV2Emphasis from './PageV2NodeKind/PageV2Emphasis.vue';
import PageV2Image from './PageV2NodeKind/PageV2Image.vue';

defineProps<{
  page: Node;
  checked?: boolean;
}>();
</script>

<template>
  <PageV2Title v-if="page['Heading'] !== undefined" :value="page" :level="page['Heading']['depth'] || 3" />
  <PageV2Text v-else-if="page['Text'] !== undefined" :value="page" />
  <PageV2Paragraph v-else-if="page['Paragraph'] !== undefined" :value="page" :checked="checked" />
  <PageV2Code v-else-if="page['Code'] !== undefined" :value="page" />
  <PageV2List v-else-if="page['List'] !== undefined" :value="page" />
  <PageV2ListItem v-else-if="page['ListItem'] !== undefined" :value="page" />
  <PageV2InlineCode v-else-if="page['InlineCode'] !== undefined" :value="page" />
  <PageV2Link v-else-if="page['Link'] !== undefined" :value="page" />
  <PageV2Strong v-else-if="page['Strong'] !== undefined" :value="page" />
  <PageV2Emphasis v-else-if="page['Emphasis'] !== undefined" :value="page" />
  <PageV2Image v-else-if="page['Image'] !== undefined" :value="page" />
  <p v-else>
    {{ page }}
  </p>
</template>