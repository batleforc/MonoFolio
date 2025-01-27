<script setup lang="ts">
import { Node } from '@portfolio/api-client';

defineProps<{
  value: Node,
}>();

</script>

<template>
  {{ value["Text"]["value"] }}
</template>
