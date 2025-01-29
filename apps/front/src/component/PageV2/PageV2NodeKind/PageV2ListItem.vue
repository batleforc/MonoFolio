<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import PageV2Content from '../PageV2Content.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <li class="pageContent" v-if="value['ListItem']['children'] !== undefined">
    <Checkbox v-if="value['ListItem']['checked'] !== undefined" binary :modelValue="value['ListItem']['checked']"
      :disabled="true" />
    <PageV2Content v-for="content in value['ListItem']['children']" :page="content" :key="content" />
  </li>
</template>
