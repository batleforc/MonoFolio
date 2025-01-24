<script setup lang="ts">
import { useIndexStore } from '../../stores';
import TextRotate from './TextRotate.vue';
import HeroLink from './HeroLink.vue'
const indexStore = useIndexStore();



</script>

<template>
  <div v-if="indexStore.inited" id="cover" class="coverPage">
    <div class="coverPageContent">
      <h1 class="coverPageTitle">{{ indexStore.homeContent.name }}</h1>
      <TextRotate :texts="indexStore.homeContent.coverTitle" />
      <div class="coverPageWrapperMedia">
        <HeroLink v-for="link in indexStore.homeContent.url" :key="link.name" :icon="link.imgUrl" :link="link.url"
          className="ico-cover" :smooth="false" :internal="false" />
      </div>
    </div>
    <HeroLink link="navbar" icon="ico#chevron-down" className="ico-next-content animate-bounce" />
  </div>
  <div v-else>
    <h1>Loading...</h1>
  </div>
</template>