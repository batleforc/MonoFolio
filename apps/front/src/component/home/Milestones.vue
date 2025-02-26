<script setup lang="ts">
import { ref } from 'vue';
import { useIndexStore } from '../../stores';
import MilestoneModal from './MilestoneModal.vue';
import IcoMoonSVG from '../helper/IcoMoonSVG.vue';

const indexStore = useIndexStore();
const modal = ref("");

const toSlug = (str: string) => {
  return str.toLowerCase().replace(/ /g, '-').replace(/[^\w-]+/g, '');
};

const extractIcon = (url: string) => {
  return url.split('#')[1] || '';
};

</script>

<template>
  <div id="milestones" class="milestones">
    <h2>Expérience</h2>
    <div class="grid ">
      <div class="timeline">
        <div class="dot"></div>
        <div class="w-[1.5px] bg-[var(--color-light-red)] h-4 grow"></div>
      </div>
      <div class="content pb-2" style="flex-direction: row;">
        <div class="text-white mr-2 flex items-center justify-center rounded-lg bg-[#482325] shrink-0 size-12"
          data-icon="Rocket" data-size="24px" data-weight="regular">
          <IcoMoonSVG icon="pacman" className="size-5 text-white" fill="currentColor" />
        </div>
        <div>
          <p>Take over the world ?</p>
          <p class="year">Now</p>
        </div>
      </div>
      <template v-for="(timeline, index) in indexStore.getHistory" :key="timeline.title">
        <div class="timeline" :to-open="'milestone' + toSlug(timeline.title)" @click="modal = timeline.title">
          <div v-if="index !== indexStore.getHistory.length - 1" class="dot"></div>
          <div class="w-[1.5px] bg-[var(--color-light-red)] h-4 "
            :class="index == indexStore.getHistory.length - 1 ? '' : 'grow'">
          </div>
          <div v-if="index === indexStore.getHistory.length - 1" class="dot"></div>
        </div>
        <div class="content cursor-pointer items-center pb-2 min-h-20 " style="flex-direction: row;"
          @click="modal = timeline.title">
          <div class="text-white flex mr-2 items-center justify-center rounded-lg bg-[#482325] shrink-0 size-12"
            data-size="24px" data-weight="regular">
            <IcoMoonSVG :icon="extractIcon(timeline.icoUrl)" className="size-5 text-white" fill="currentColor" />
          </div>
          <div>
            <p>{{ timeline.title }}</p>
            <p class="year">{{ timeline.date }} - {{ timeline.lieux }}</p>
          </div>
        </div>
      </template>
    </div>
  </div>
  <MilestoneModal v-if="modal !== undefined && modal !== ''" :modal="indexStore.getHomeHistory(modal)"
    :close="() => { modal = '' }" />
</template>