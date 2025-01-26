<script setup lang="ts">
import { Node } from '@portfolio/api-client';

defineProps<{
  page: Node;
}>();
</script>

<template>
  <div class="pageContent">
    {{ page }}
  </div>
</template>