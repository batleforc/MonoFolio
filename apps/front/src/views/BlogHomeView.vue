<script setup lang="ts">
import PageMetadata from '../component/Page/PageMetadata.vue';
import { useBlogStore } from '../stores/blog';
import { useDocStore } from '../stores/doc';


const docStore = useDocStore();
const blogStore = useBlogStore();

if (!docStore.inited && !docStore.docLoading) {
  docStore.init();
}
if (!blogStore.inited && !blogStore.blogLoading) {
  blogStore.init();
}


</script>

<template>
  <div id="title" class="container">
    <div v-if="blogStore.inited && !blogStore.blogLoading && blogStore.getBlogContent !== undefined">
      <h2>Blog posts</h2>
      <ul>
        <li class="py-2" v-for="post in blogStore.getBlogContent" :key="post[0]">
          <PageMetadata :default-collapsed="true" :metadata="post[1].metadata">
            <template #footer>
              <div class="flex flex-col place-items-end	">
                <RouterLink :to="{ name: 'blogcontent', params: { page: post[0] } }">
                  <Button label="Read more" />
                </RouterLink>
              </div>
            </template>
          </PageMetadata>

        </li>
      </ul>
    </div>
  </div>
</template>

<style lang="scss">
.blogHomeView {
  @apply text-4xl;
}
</style>