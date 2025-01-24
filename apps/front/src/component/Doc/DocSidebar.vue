<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useDocStore } from '../../stores/doc';
import DocSidebarItem from './DocSidebarItem.vue';
import { ref, watch } from 'vue';
import IcoMoonSVG from '../helper/IcoMoonSVG.vue';

const route = useRoute();
const docStore = useDocStore();
let path = ref(Array.isArray(route.params.page) ? route.params.page : [route.params.page]);

watch(() => route.params.page, (newValue, oldValue) => {
  if (newValue !== oldValue) {
    path.value = Array.isArray(newValue) ? newValue : [newValue];
  }
});

let showMobileSidebar = ref(false);

</script>

<template>
  <aside class="docSidebar" :class="showMobileSidebar ? 'docSidebarShowMobile' : ''">
    <RouterLink :to="{ name: 'doc' }">Doc Home</RouterLink>
    <DocSidebarItem :path="path" v-for="categorie in docStore.docContent.sub_categories" v-bind:key="categorie.name"
      :docContent="categorie" />
  </aside>
  <div class="docSidebarMobile">
    <button @click="showMobileSidebar = !showMobileSidebar">
      <IcoMoonSVG icon="burger" />
    </button>
  </div>
</template>
