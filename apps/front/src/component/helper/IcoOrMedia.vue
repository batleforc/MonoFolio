<script setup lang="ts">
import IcoMoonSVG from './IcoMoonSVG.vue';
defineProps({
  media: {
    type: String,
    required: true,
  },
  className: {
    type: String,
    default: '',
  },
});

</script>

<template>
  <IcoMoonSVG v-if="media.includes('ico#')" :icon="media.replace('ico#', '')" :className="className" />
  <p v-else>{{ media }}</p>
</template>