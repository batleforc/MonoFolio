<script setup lang="ts">
import { useIndexStore } from '../../stores';
import TextRotate from './TextRotate.vue';
import HeroLink from './HeroLink.vue'
import { AreWeHome } from '../../helper';
const indexStore = useIndexStore();

const smoothScroll = (event: MouseEvent) => {
    if (!event) return;
    event.preventDefault();
    document.querySelector(AreWeHome() ? "#about" : "#title")?.scrollIntoView({ behavior: 'smooth' });
}

</script>

<template>
    <div v-if="indexStore.inited" id="cover" class="coverPage">
        <div class="coverPageContent">
            <h1 class="coverPageTitle">{{ indexStore.homeContent.name }}</h1>
            <TextRotate :texts="indexStore.homeContent.coverTitle" />
            <div class="coverPageWrapperMedia">
                <HeroLink v-for="link in indexStore.homeContent.url" :key="link.name" :icon="link.imgUrl"
                    :link="link.url" className="ico-cover" />
            </div>
        </div>
        <HeroLink :link="AreWeHome() ? 'about' : 'title'" icon="ico#chevron-down"
            className="ico-next-content animate-bounce" :func="smoothScroll" />
    </div>
    <div v-else>
        <h1>Loading...</h1>
    </div>
</template>

<style lang="scss">
.coverPage {
    height: 100vh;
    background-color: #1b203a;
    color: #f2f4f3;
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
    color: #f2f4f3;
    border-radius: 50%;
    border: 1px solid #f2f4f3;
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
    color: #f2f4f3;
}
</style>