<script lang="ts" setup>
import { Block } from '../../markdown';


defineProps<{
  content: Block;
}>();
</script>

<template>
  <div class="pageString" v-html="content.value" />
</template>

<style></style>