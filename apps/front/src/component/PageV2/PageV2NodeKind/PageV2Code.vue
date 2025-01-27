<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import VueMermaidString from 'vue-mermaid-string';


defineProps<{
  value: Node,
}>();

</script>

<template>
  <div class="pageContent" v-if="value['Code']['value'] !== undefined">
    <VueMermaidString v-if="value['Code']['lang'] === 'mermaid'" class="py-2 pageCodeMermaid"
      :value="value['Code']['value']" />
    <highlightjs v-else class="py-2 rounded pageHljs" autodetect :code="value['Code']['value']" />
  </div>
</template>
