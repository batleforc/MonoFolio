<script setup lang="ts">
import { BlockType, TitleBlock } from '../../markdown';
import PageString from './PageString.vue';
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
      <PageString v-if="line.type === BlockType.string" :content="line" />
      <VueMermaidString class="py-2 pageCodeMermaid"
        v-else-if="line.type === BlockType.startEndCodeBlock && line.additionalValue.code === 'mermaid'"
        :value="line.value" />
      <highlightjs class="py-2 rounded pageHljs" v-else-if="line.type === BlockType.startEndCodeBlock" autodetect
        :code="line.value" />
      <div class="flex space-x-2" v-else-if="line.type === BlockType.checkbox">
        <p v-for="indent in Number(line.additionalValue.indent)" :key="indent" class="pageContentIndent">
        </p>
        <ul class="list-disc">
          <li>
            <div class="flex space-x-2">
              <Checkbox binary trueValue="checked" falseValue="unchecked" :modelValue="line.additionalValue.checked"
                :disabled="true" />
              <p>{{ line.value }} </p>
            </div>
          </li>
        </ul>

      </div>
      <div class="flex space-x-2" v-else-if="line.type === BlockType.list">
        <p v-for="indent in line.additionalValue.indent" :key="indent" class="pageContentIndent"></p>
        <ul class="list-disc">
          <li>{{ line.value }}</li>
        </ul>
      </div>
    </div>
    <div class="pageSubBlock">
      <PageContent v-for="titleBlock in content.subTitleBlock" :key="titleBlock.value" :content="titleBlock" />
    </div>
  </div>
</template>

<style lang="scss">
.pageHljs code {
  @apply rounded-xl;
}

.pageContentIndent {
  @apply w-1;
}
</style>