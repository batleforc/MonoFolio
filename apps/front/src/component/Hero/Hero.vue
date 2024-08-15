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
                <HeroLink v-for="link in indexStore.homeContent.url" :key="link.name" :icon="link.imgUrl"
                    :link="link.url" className="ico-cover" :smooth="false" :internal="false" />
            </div>
        </div>
        <HeroLink link="navbar" icon="ico#chevron-down" className="ico-next-content animate-bounce" />
    </div>
    <div v-else>
        <h1>Loading...</h1>
    </div>
</template>

<style lang="scss">
@import '../../var.scss';

.coverPage {
    height: 100vh;
    background-color: $color-bgCover;
    color: $color-textCover;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.coverPageTitle {
    text-align: center;
    font-size: 3.75em;
    font-weight: inherit;
    line-height: 1;
    margin: 0;
}

.coverPageContent>.txt-rotate-wrapper {
    display: flex;
}

.coverPageWrapperMedia {
    display: flex;
    justify-content: center;
    place-items: center;
}

.ico-cover {
    font-size: 38px;
    color: $color-textCover;
    border-radius: 50%;
    border: 1px solid $color-textCover;
    padding: 5px;
    margin: 5px;
}

.ico-next-content {
    position: absolute;
    bottom: 10%;
    left: 0;
    right: 0;
    margin-left: auto;
    margin-right: auto;
    width: 30px;
    font-size: 30px;
    color: $color-textCover;
}
</style>