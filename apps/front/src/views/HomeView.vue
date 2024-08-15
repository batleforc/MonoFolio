<script setup lang="ts">
import { useIndexStore } from '../stores';
import { getMediaApiUrl } from '../helper/index.ts';
import IcoOrMedia from '../component/helper/IcoOrMedia.vue';
const indexStore = useIndexStore();
</script>

<template>
    <div v-if="indexStore.inited" class="homePageContainer">
        <Panel header="Qui suis je ?">
            <p class="m-0">{{ indexStore.homeContent.presentation }}</p>
        </Panel>
        <div class="homePageCvContainer">
            <a :href="getMediaApiUrl(indexStore.homeContent.cvUrl)" target="_blank" rel="noreferrer" class="homePageCV">
                Mon CV
                <IcoOrMedia media="ico#download" className="ml-2" />
            </a>
        </div>
    </div>
</template>

<style lang="scss">
@import '../var.scss';

.homePageContainer {
    @apply flex justify-center flex-col;
}

.homePageCV {
    @apply border-2 rounded-full px-5 py-2;
    background-color: $color-bgCover;
    color: $color-textCover;
}

.homePageCvContainer {
    @apply my-8 flex justify-center;
}
</style>