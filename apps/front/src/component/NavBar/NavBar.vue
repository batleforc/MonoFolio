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
      <HeroLink :link="'/'" :icon="'ico#home'" className="ico-navBarHeader " :smooth="false" :internal="true"
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


<style lang="scss">
@use "../../var.scss" as *;

.navBarHeaderItem:hover {
  background-color: $color-bgCoverLighter;
  @apply rounded-lg;
}

.p-menubar-button {
  background-color: $color-bgCover;
  color: $color-textCover;
  @apply h-10 w-10;

  .p-icon {
    @apply h-8 w-8 p-1 rounded-lg;
  }

  svg:hover {
    background-color: $color-bgCoverLighter;
  }
}

.p-menubar-item.p-focus>.p-menubar-item-content {
  background-color: $color-bgCoverLighter;
}

.p-menubar-item:not(.p-disabled)>.p-menubar-item-content:hover {
  background-color: $color-bgCoverLighter;
}

.p-menubar-root-list {
  background-color: $color-bgCover;
  @apply p-0.5 rounded-b-lg;

  &:focus {
    @apply focus:outline-none;
    background-color: $color-bgCoverLighter;
  }
}

.navBarHeader {
  background-color: $color-bgCover;
  z-index: 2000;
  @apply rounded-none sticky top-0 justify-between h-16;

  @media (min-width: 740px) {
    @apply justify-center;
  }

  .p-menubar-item-content {
    @apply p-0.5;
  }

  .p-focus:hover {
    background-color: $color-bgCoverLighter;
  }
}

.navBarHeader.p-menubar-mobile-active {
  @apply rounded-b-none;
}

.navBarHeaderLabel {
  @apply ml-2;
  color: $color-textCover;
}

.ico-navBarHeader {
  font-size: 38px;
  color: $color-textCover;

}
</style>