<script setup lang="ts">
import { useIndexStore } from '../../stores';
import HeroLink from '../Hero/HeroLink.vue';
import { getMediaApiUrl } from '../../helper/index';
const indexStore = useIndexStore();
</script>

<template>
  <div class="contact-container">
    <h2>Contact</h2>
    <div class="contact">
      <div class="item">
        <p class="label">Location</p>
        <p>France, Somewhere or Nowhere</p>
      </div>
      <div class="item">
        <p class="label">Availability</p>
        <p>
          Non disponible, Ingénieur Couche d'échange DevOps Macif
        </p>
      </div>
      <div class="item">
        <p class="label">Email</p>
        <p>maxime.lerichepro@gmail.com</p>
      </div>
      <div class="item">
        <p class="label">Réseau Sociaux</p>
        <div class="flex">
          <HeroLink v-for="link in indexStore.homeContent.url" :key="link.name" linkClassName="contact-social-container"
            :icon="link.imgUrl" :link="link.url" className="text-white contact-social" :smooth="false" :internal="false"
            fill="currentColor" />
        </div>
      </div>
      <div class="item full">
        <p class="label">CV</p>
        <div class="flex">
          <a :href="getMediaApiUrl(indexStore.homeContent.cvUrl)" target="_blank" rel="noreferrer" class="homePageCV">
            Mon CV
            <IcoOrMedia media="ico#download" className="ml-2" />
          </a>
        </div>
      </div>
    </div>
  </div>
</template>