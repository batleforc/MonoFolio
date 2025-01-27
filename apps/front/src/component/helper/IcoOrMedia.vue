<script setup lang="ts">
import { getMediaApiUrl } from '../../helper';
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
  alt: {
    type: String,
    default: '',
  },
});

</script>

<template>
  <IcoMoonSVG v-if="media.startsWith('ico#')" :icon="media.replace('ico#', '')" :className="className" />
  <img :alt="alt" v-else-if="media.startsWith('media#')" :src="getMediaApiUrl(media.replace('media#', ''))"
    :class="className" />
  <img :alt="alt" v-else-if="media.startsWith('http') || media.startsWith('url#')" :src="media.replace('media#', '')"
    :class="className" />
  <p v-else>{{ media }}</p>
</template>