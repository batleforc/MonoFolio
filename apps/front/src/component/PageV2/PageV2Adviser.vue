<script setup lang="ts">
import { DocHeader } from '@portfolio/api-client';
import IcoOrMedia from '../helper/IcoOrMedia.vue';
import { getMediaUrl, MediaType } from '../../helper';
import { ref } from 'vue';
const props = defineProps<{
  metadata: DocHeader;
  defaultCollapsed?: boolean;
}>();

let picture = ref(getMediaUrl(props.metadata.image || ""));

</script>

<template>
  <div class="blogListe content">
    <div v-if="picture.type === MediaType.ApiMedia || picture.type === MediaType.Url || picture.type === MediaType.Http"
      class="image" :style="`background-image: url('${picture.url}');`">
    </div>
    <div v-else-if="picture.type === MediaType.None || picture.type === MediaType.Ico" class="image-none"
      style='background-image: url("https://cdn.usegalileo.ai/sdxl10/e949dae8-f964-4d75-84b0-e6d5756e1b80.png");'>
    </div>
    <h1>{{ metadata.title }}</h1>

  </div>

</template>