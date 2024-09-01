<script setup lang="ts">
import { TitleBlock } from '../../markdown';
import PageTitle from './PageTitle.vue';

defineProps<{
    content: TitleBlock;
}>();
</script>

<template>
    <div class="pageContent">
        <PageTitle :value="content.value" :level="content.level" />
        <p v-for="line in content.subBlocks" :key="line.value">{{ line.value }} - {{ line.type }}</p>
        <div class="pageSubBlock">
            <PageContent v-for="titleBlock in content.subTitleBlock" :key="titleBlock.value" :content="titleBlock" />
        </div>
    </div>
</template>

<style lang="scss"></style>