<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
  level: number,
}>();

const computeId = (value: Node) => {
  if (value['Heading']['children'][0] !== undefined && value['Heading']['children'][0]['Text'] !== undefined)
    return value['Heading']['children'][0]['Text']['value'].replace(/ /g, '-').toLowerCase();
  return '';
};

</script>

<template>
  <div class="pageBlockTitle">
    <h1 :id="computeId(value)" class="pageBlockTitleH pageBlockTitleH1" v-if="level === 1">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h1>
    <h2 :id="computeId(value)" class="pageBlockTitleH pageBlockTitleH2" v-else-if="level === 2">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h2>
    <h3 :id="computeId(value)" class="pageBlockTitleH pageBlockTitleH3" v-else-if="level === 3">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h3>
    <h4 :id="computeId(value)" class="pageBlockTitleH pageBlockTitleH4" v-else>
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h4>
  </div>
</template>
