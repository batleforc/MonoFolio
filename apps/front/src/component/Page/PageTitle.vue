<script setup lang="ts">
defineProps<{
    value: string,
    level: number,
}>();
</script>

<template>
    <div class="pageBlockTitle">
        <h1 class="pageBlockTitleH pageBlockTitleH1" v-if="level === 1">{{ value }}</h1>
        <h2 class="pageBlockTitleH pageBlockTitleH2" v-else-if="level === 2">{{ value }}</h2>
        <h3 class="pageBlockTitleH pageBlockTitleH3" v-else-if="level === 3">{{ value }}</h3>
        <h4 class="pageBlockTitleH pageBlockTitleH4" v-else>{{ value }}</h4>
    </div>
</template>

<style lang="scss">
.pageBlockTitleH {
    @apply m-2 tracking-wide;
}

.pageBlockTitleH1 {
    @apply font-bold text-5xl;
}

.pageBlockTitleH2 {
    @apply font-bold text-4xl;
}

.pageBlockTitleH3 {
    @apply font-semibold text-3xl;
}

.pageBlockTitleH4 {
    @apply font-medium text-2xl;
}
</style>