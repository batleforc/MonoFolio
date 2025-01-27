<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';


defineProps<{
  value: Node,
}>();

</script>

<template>
  <span class="font-bold " v-if="value['Strong']['children'] !== undefined">
    <PageV2Content v-for="content in value['Strong']['children']" :page="content" :key="content" />
  </span>
</template>
