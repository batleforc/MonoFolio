<script setup lang="ts">
import WarnBan from '../component/helper/WarnBan.vue';
import PageMetadata from '../component/Page/PageMetadata.vue';
import { useBlogStore } from '../stores/blog';
import { useDocStore } from '../stores/doc';
import { usePageStore } from '../stores/page';
import Footer from '../component/helper/Footer.vue';


const docStore = useDocStore();
const blogStore = useBlogStore();
const pageStore = usePageStore();

if (!docStore.inited && !docStore.docLoading) {
  docStore.init();
}
if (!blogStore.inited && !blogStore.blogLoading) {
  blogStore.init();
}

</script>

<template>
  <div id="title">
    <WarnBan v-if="pageStore.maintenance" text="Maintenance en cours" />
    <div class="flex flex-row justify-center"
      v-if="blogStore.inited && !blogStore.blogLoading && blogStore.getBlogContent !== undefined">
      <ul class="flex flex-col w-full">
        <li class="p-2" v-for="post in blogStore.getBlogContent" :key="post[0]">
          <PageMetadata :default-collapsed="true" :metadata="post[1].metadata">
            <template #footer>
              <div class="flex flex-col place-items-end	">
                <RouterLink :to="{ name: 'blogcontent', params: { page: post[0] } }">
                  <Button class="blogHomeButton" label="Read more" />
                </RouterLink>
              </div>
            </template>
          </PageMetadata>
        </li>
      </ul>
    </div>
    <Footer className="docFooter" />
  </div>
</template>
