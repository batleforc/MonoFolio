<script setup lang="ts">
import { useIndexStore } from '../../stores';

const indexStore = useIndexStore();
</script>

<template>
  <div class="milestones">
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
        <div class="timeline">
          <div v-if="index !== indexStore.getHistory.length - 1" class="dot"></div>
          <div class="w-[1.5px] bg-[var(--color-light-red)] h-4"
            :class="index == indexStore.getHistory.length - 1 ? '' : 'grow'">
          </div>
          <div v-if="index === indexStore.getHistory.length - 1" class="dot"></div>
        </div>
        <div class="content">
          <p>{{ timeline.title }}</p>
          <p class="year">{{ timeline.date }} - {{ timeline.lieux }}</p>
        </div>
      </template>
    </div>
  </div>
</template>