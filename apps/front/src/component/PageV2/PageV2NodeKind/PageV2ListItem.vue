<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <li class="pageContent" v-if="value['ListItem']['children'] !== undefined">
    <PageV2Content v-for="content in value['ListItem']['children']" :page="content" :key="content" />
  </li>
</template>
