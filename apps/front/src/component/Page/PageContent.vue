<script setup lang="ts">
import { BlockType, TitleBlock } from '../../markdown';
import PageTitle from './PageTitle.vue';
import VueMermaidString from 'vue-mermaid-string';


defineProps<{
    content: TitleBlock;
}>();
</script>

<template>
    <div class="pageContent">
        <PageTitle :value="content.value" :level="content.level" />
        <div v-for="line in content.subBlocks" :key="line.value">
            <p v-if="line.type === BlockType.string">{{ line.value }}</p>
            <VueMermaidString class="py-2 pageCodeMermaid"
                v-else-if="line.type === BlockType.startEndCodeBlock && line.additionalValue === 'mermaid'"
                :value="line.value" />
            <highlightjs class="py-2" v-else-if="line.type === BlockType.startEndCodeBlock" autodetect
                :code="line.value" />
            <div class="flex space-x-2" v-else-if="line.type === BlockType.checkbox">
                <Checkbox binary trueValue="checked" falseValue="unchecked" :modelValue="line.additionalValue"
                    :disabled="true" />
                <p>{{ line.value }} - {{ line.additionalValue }}</p>
            </div>
        </div>
        <div class="pageSubBlock">
            <PageContent v-for="titleBlock in content.subTitleBlock" :key="titleBlock.value" :content="titleBlock" />
        </div>
    </div>
</template>

<style lang="scss"></style>