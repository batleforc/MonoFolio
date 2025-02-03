<script setup lang="ts">
import { useIndexStore } from '../stores';
import { getMediaApiUrl } from '../helper/index';
import IcoOrMedia from '../component/helper/IcoOrMedia.vue';
import Footer from '../component/helper/Footer.vue';
import { useBreakpoints } from '../helper/mediaQuerry';
const indexStore = useIndexStore();
const { type } = useBreakpoints();
</script>

<template>
  <div v-if="indexStore.isInitialized" class="homePageContainer flex flex-col items-center">
    <Panel class="homePageWhoAmI w-3/4" header="Qui suis je ?">
      <p v-for="text in indexStore.getPresentation" :key="text">{{ text }}</p>
    </Panel>
    <div class="homePageCvContainer">
      <a :href="getMediaApiUrl(indexStore.homeContent.cvUrl)" target="_blank" rel="noreferrer" class="homePageCV">
        Mon CV
        <IcoOrMedia media="ico#download" className="ml-2" />
      </a>
    </div>
    <div class="flex justify-center w-3/4">
      <Timeline :value="indexStore.getHistory" :align="type == 'xs' ? 'left' : 'alternate'"
        class="w-full px-2 py-5 customized-timeline " :class="type == 'xs' ? 'justify-end' : ''">
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
              <p v-for="description in slotProps.item.description.split('\n')" :key="description">
                {{ description }}
              </p>
            </template>
          </Card>
        </template>
      </Timeline>
    </div>
    <Panel class="w-3/4" header="Pour aller plus loin">
      <p>Maintenant que vous avez un peu plus d'informations sur mon historique, que diriez-vous de découvrir la
        suite ?
      </p>
      <div class="flex justify-around flex-wrap">
        <RouterLink :to="{ name: 'doc' }">
          <Card class="homePageGoFurtherCard">
            <template #title>
              <h3>Ma doc</h3>
            </template>
            <template #content>
              <p>J'essaye de regrouper progressivement mes documentation ici</p>
            </template>
          </Card>
        </RouterLink>
        <RouterLink :to="{ name: 'doccontent', params: { page: ['Techno'] } }">
          <Card class="homePageGoFurtherCard">
            <template #title>
              <h3>Les techno</h3>
            </template>
            <template #content>
              <p>
                J'essaye de regrouper et maintenir une base des technologies que j'utilise ici
              </p>
            </template>
          </Card>
        </RouterLink>
        <RouterLink :to="{ name: 'doccontent', params: { page: ['Project'] } }">
          <Card class="homePageGoFurtherCard">
            <template #title>
              <h3>Les Projet</h3>
            </template>
            <template #content>
              <p>J'essaye de r&eacute;f&eacute;rencer mes projet ici</p>
            </template>
          </Card>
        </RouterLink>
        <RouterLink :to="{ name: 'blog' }">
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
      <Footer className="docFooter" />
    </Panel>
  </div>
</template>