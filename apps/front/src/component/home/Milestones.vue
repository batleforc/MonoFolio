<script setup lang="ts">
import { ref } from 'vue';
import { useIndexStore } from '../../stores';
import MilestoneModal from './MilestoneModal.vue';

const indexStore = useIndexStore();
const modal = ref("");

const toSlug = (str: string) => {
  return str.toLowerCase().replace(/ /g, '-').replace(/[^\w-]+/g, '');
};

</script>

<template>
  <div id="milestones" class="milestones">
    <h2>Expérience</h2>
    <div class="grid">
      <div class="timeline">
        <div class="dot"></div>
        <div class="w-[1.5px] bg-[var(--color-light-red)] h-4 grow"></div>
      </div>
      <div class="content">
        <p>Take over the world ?</p>
        <p class="year">Now</p>
      </div>
      <template v-for="(timeline, index) in indexStore.getHistory" :key="timeline.title">
        <div class="timeline" :to-open="'milestone' + toSlug(timeline.title)" @click="modal = timeline.title">
          <div v-if="index !== indexStore.getHistory.length - 1" class="dot"></div>
          <div class="w-[1.5px] bg-[var(--color-light-red)] h-4 "
            :class="index == indexStore.getHistory.length - 1 ? '' : 'grow'">
          </div>
          <div v-if="index === indexStore.getHistory.length - 1" class="dot"></div>
        </div>
        <div class="content cursor-pointer" @click="modal = timeline.title">
          <p>{{ timeline.title }}</p>
          <p class="year">{{ timeline.date }} - {{ timeline.lieux }}</p>
        </div>
      </template>
    </div>

  </div>
  <MilestoneModal v-if="modal !== undefined && modal !== ''" :modal="indexStore.getHomeHistory(modal)"
    :close="() => { modal = '' }" />
</template>