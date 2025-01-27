<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';


defineProps<{
  value: Node,
}>();

</script>

<template>
  <span class="italic" v-if="value['Emphasis']['children'] !== undefined">
    <PageV2Content v-for="content in value['Emphasis']['children']" :page="content" :key="content" />
  </span>
</template>
