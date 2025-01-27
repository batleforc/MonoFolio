<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
  level: number,
}>();

</script>

<template>
  <div class="pageBlockTitle">
    <h1 class="pageBlockTitleH pageBlockTitleH1" v-if="level === 1">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h1>
    <h2 class="pageBlockTitleH pageBlockTitleH2" v-else-if="level === 2">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h2>
    <h3 class="pageBlockTitleH pageBlockTitleH3" v-else-if="level === 3">
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h3>
    <h4 class="pageBlockTitleH pageBlockTitleH4" v-else>
      <PageV2Content v-for="content in value['Heading']['children']" :page="content" :key="content" />
    </h4>
  </div>
</template>
