<script setup lang="ts">
import Menubar from 'primevue/menubar';
import HeroLink from '../Hero/HeroLink.vue';

const props = defineProps({
  link: {
    type: Array<{
      imgUrl: string,
      name: string,
      url: string
    }>,
    required: false
  }
});

let link_swap = props.link !== undefined ? props.link?.map((item) => {
  return {
    icon: item.imgUrl,
    label: item.name,
    to: item.url,
    internal: item.url.startsWith('/') || item.url.includes('/') && !item.url.includes('http'),
    smooth: !item.url.includes('http')
  }
}) : [];

const items = [
  {
    icon: 'ico#files-empty',
    label: 'Doc',
    to: '/doc',
    internal: true,
    smooth: false,
  },
  {
    icon: 'ico#newspaper',
    label: 'Blog',
    to: '/blog',
    internal: true,
    smooth: false,
  },
  ...link_swap
]


</script>

<template>
  <Menubar id="navbar" breakpoint="740px" class="navBarHeader" :model="items" @focus="() => { }">
    <template #start>
      <HeroLink :link="'/'" :icon="'ico#home'" className="ico-navBarHeader" :smooth="false" :internal="true"
        label="Maxime Leriche" labelClassName="navBarHeaderLabel"
        linkClassName="flex items-center navBarHeaderHome navBarHeaderItem" />
    </template>
    <template #item="{ item }">
      <HeroLink :link="item.to" :icon="item.icon" className="ico-navBarHeader" :smooth="item.smooth"
        :internal="item.internal" :label="String(item.label)" labelClassName="navBarHeaderLabel"
        linkClassName="flex items-center navBarHeaderItem" />
    </template>
  </Menubar>
</template>
