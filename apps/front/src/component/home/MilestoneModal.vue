<script setup lang="ts">
import { HomeHistory } from '@portfolio/api-client';
import { getMediaUrl, MediaType } from '../../helper';
import { ref } from 'vue';

const props = defineProps<{ modal: HomeHistory | undefined, close: () => void }>();

const picture = ref(getMediaUrl(props.modal?.imgUrl));

</script>

<template>
  <div class="milestonesModal" v-if="modal !== undefined">
    <div class="header">
      <div @click="close" class="close-icon" data-icon="X" data-size="24px" data-weight="regular">
        <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
          <path
            d="M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z">
          </path>
        </svg>
      </div>
      <h2>{{ modal.title }}</h2>
    </div>
    <div class="container">
      <div class="content">
        <div
          v-if="picture.type === MediaType.ApiMedia || picture.type === MediaType.Url || picture.type === MediaType.Http"
          class="image" :style="`background-image: url('${picture.url}');`">
        </div>
        <div v-else-if="picture.type === MediaType.None || picture.type === MediaType.Ico" class="image-none"
          style='background-image: url("https://cdn.usegalileo.ai/sdxl10/e949dae8-f964-4d75-84b0-e6d5756e1b80.png");'>
        </div>
      </div>
      <h1>{{ modal.lieux }}</h1>
      <p class="date">{{ modal.date }}</p>
      <p class="description">{{ modal.description }}</p>
    </div>
    <div class="h-17 bg-[#34191a]"></div>
  </div>
</template>