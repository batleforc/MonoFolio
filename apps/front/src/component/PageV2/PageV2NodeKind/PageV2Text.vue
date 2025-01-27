<script setup lang="ts">
import { Node } from '@portfolio/api-client';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <div class="pageString">
    {{ value["Text"]["value"] }}
  </div>
</template>
