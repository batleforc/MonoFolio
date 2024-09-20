<script setup lang="ts">
import { DocHeader } from '@portfolio/api-client';
import PageTechnoChip from './PageTechnoChip.vue';
import IcoOrMedia from '../helper/IcoOrMedia.vue';
defineProps<{
  metadata: DocHeader;
}>();

</script>

<template>
  <div class="pageHeader">
    <Card>
      <template #header>
        <div class="flex justify-between px-4 pt-2">
          <a target="_blank" rel="noreferrer" :href="metadata.writter.url" class="flex items-center gap-2">
            <Avatar :image="metadata.writter.avatar" size="xlarge" />
            <p class="font-bold text-2xl">{{ metadata.writter.name }}</p>
          </a>
          <div>
            <p>{{ new Date(metadata.date).toLocaleString() }}</p>
          </div>
        </div>
      </template>
      <template #title>
        <h1>
          <IcoOrMedia v-if="metadata.image !== undefined && metadata.image !== null && metadata.image !== ''"
            className="pageHeaderMetadataIco" :media="metadata.image" />
          {{ metadata.title }}
        </h1>
      </template>
      <template #subtitle>
        <p>{{ metadata.description }}</p>
      </template>
      <template #content>
        <Panel v-if="metadata.techno !== undefined && metadata.techno.length > 0" toggleable header="Techno">
          <div class="pageHeaderTechno">
            <PageTechnoChip v-for="techno in metadata.techno" :key="techno" :technoName="techno" />
          </div>
        </Panel>
        <Panel v-if="metadata.tags !== undefined && metadata.tags.length > 0" toggleable header="Tag">
          <div class="pageHeaderTag">
            <Chip v-for="tag in metadata.tags" :key="tag" :label="tag" />
          </div>
        </Panel>
        <Panel v-if="metadata.links !== undefined && metadata.links.length > 0" toggleable header="Link">
          <div class="pageHeaderLink">
            <a v-for="link in metadata.links" :key="link.url" :href="link.url" target="_blank">
              <Chip :label="link.name" />
            </a>
          </div>
        </Panel>
      </template>
    </Card>
  </div>
</template>

<style lang="scss">
@import '../../var.scss';

.pageHeaderTechno,
.pageHeaderTag,
.pageHeaderLink {
  @apply flex flex-wrap gap-2;
}
</style>