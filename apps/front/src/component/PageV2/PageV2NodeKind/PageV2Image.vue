<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import IcoOrMedia from '../../helper/IcoOrMedia.vue';

defineProps<{
  value: Node,
}>();

</script>

<template>
  <div class="pageContent items-center flex justify-center" v-if="value['Image'] !== undefined">
    <IcoOrMedia :media="value['Image']['url']" :alt="value['Image']['alt']" className="pageImage" />
  </div>
</template>
