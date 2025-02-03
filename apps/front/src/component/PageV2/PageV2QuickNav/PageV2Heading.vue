<script setup lang="ts">
import { Node } from '@portfolio/api-client';
import { useRouter } from 'vue-router'

const router = useRouter()

defineProps<{
  page: Node
}>();

const computeId = (value: Node) => {
  if (value['Heading']['children'][0] !== undefined && value['Heading']['children'][0]['Text'] !== undefined)
    return '#' + value['Heading']['children'][0]['Text']['value'].replace(/ /g, '-').toLowerCase();
  return '';
};

const smoothScroll = (event: MouseEvent) => {
  if (!event) return;
  event.preventDefault();
  const href = (event.target as HTMLElement).getAttribute('href')
  document.querySelector(href)?.scrollIntoView({ behavior: 'smooth' });
  router.push({ hash: href });
}

</script>

<template>
  <a :href="computeId(page)" @click="(event) => smoothScroll(event)" class="pageQuickNavHeading"
    :class="'pageQuickNavHeadingH' + page['Heading']['depth']">{{
      page["Heading"]["children"][0]["Text"]["value"] }}</a>
</template>