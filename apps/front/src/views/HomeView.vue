<script setup lang="ts">
import { useIndexStore } from '../stores';
import { getMediaApiUrl } from '../helper/index';
import IcoOrMedia from '../component/helper/IcoOrMedia.vue';
import { useBreakpoints } from '../helper/mediaQuerry';
const indexStore = useIndexStore();
const { type } = useBreakpoints();
</script>

<template>
    <div v-if="indexStore.inited" class="homePageContainer">
        <Panel class="homePageWhoAmI" header="Qui suis je ?">
            <p v-for="text in indexStore.getPresentation" :key="text">{{ text }}</p>
        </Panel>
        <div class="homePageCvContainer">
            <a :href="getMediaApiUrl(indexStore.homeContent.cvUrl)" target="_blank" rel="noreferrer" class="homePageCV">
                Mon CV
                <IcoOrMedia media="ico#download" className="ml-2" />
            </a>
        </div>
        <div>
            <Timeline :value="indexStore.getHistory" :align="type == 'xs' ? 'left' : 'alternate'"
                class="w-full px-2 py-5 customized-timeline" :class="type == 'xs' ? 'justify-end' : ''">
                <template #content="slotProps">
                    <Card class="min-w-64 pt-4 text-pretty">
                        <template #header>
                            <div class="flex justify-center">
                                <img :src="getMediaApiUrl(slotProps.item.imgUrl)" alt="img" class="w-20 h-20" />
                            </div>
                        </template>
                        <template #title>
                            {{ slotProps.item.title }}
                        </template>
                        <template #subtitle>
                            {{ slotProps.item.lieux }}, {{ slotProps.item.date }}
                        </template>
                        <template #content>
                            <p v-for="description in slotProps.item.description.split('\n')" :key="description">{{
                                description }}</p>
                        </template>
                    </Card>
                </template>
            </Timeline>
        </div>
        <Panel class="" header="Pour aller plus loin">
            <p>Maintenant que vous avez un peu plus d'informations sur mon historique, que diriez-vous de découvrir la
                suite ?
            </p>
            <div class="flex justify-around flex-wrap">
                <RouterLink to="doc">
                    <Card class="homePageGoFurtherCard">
                        <template #title>
                            <h3>Ma doc</h3>
                        </template>
                        <template #content>
                            <p>J'essaye de regrouper progressivement mes documentation ici</p>
                        </template>
                    </Card>
                </RouterLink>
                <RouterLink to="doc">
                    <Card class="homePageGoFurtherCard">
                        <template #title>
                            <h3>Les Projet</h3>
                        </template>
                        <template #content>
                            <p>J'essaye de r&eacute;f&eacute;rencer mes projet ici</p>
                        </template>
                    </Card>
                </RouterLink>
                <RouterLink to="blog">
                    <Card class="homePageGoFurtherCard">
                        <template #title>
                            <h3>Mon espace "Blog"</h3>
                        </template>
                        <template #content>
                            <p>Vous retrouverez les dernières informations ici</p>
                        </template>
                    </Card>
                </RouterLink>
            </div>
        </Panel>
    </div>
</template>

<style lang="scss">
@import '../var.scss';

.homePageGoFurtherCard {
    @apply w-64 m-2;
}

.homePageContainer {
    @apply flex justify-center flex-col;
}

.homePageWhoAmI {
    @apply my-4 mx-2;
}

.homePageCV {
    @apply border-2 rounded-full px-5 py-2;
    background-color: $color-bgCover;
    color: $color-textCover;
}

.homePageCvContainer {
    @apply my-8 flex justify-center;
}

.p-timeline-event-content {
    @apply pb-10;
}
</style>
