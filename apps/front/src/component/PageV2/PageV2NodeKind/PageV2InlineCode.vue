<script setup lang="ts">
import { Node } from '@portfolio/api-client';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <span class="pageInlineCode bg-gray-400 p-1 rounded italic mx-1" v-if="value['InlineCode']['value'] !== undefined">
    {{ value['InlineCode']['value'] }}
  </span>
</template>
