<script setup lang="ts">
import { useRoute } from 'vue-router';
import IcoMoonSVG from '../helper/IcoMoonSVG.vue';

const route = useRoute();

</script>

<template>
  <footer>
    <RouterLink :to="{ name: 'home' }" :class="route.name === 'home' ? 'active' : 'inactive'">
      <div class="flex h-8 items-center justify-center" data-icon="House" data-size="24px" data-weight="fill">
        <IcoMoonSVG icon="home" fill="currentColor" className="w-24 " />
      </div>
      <p>Home</p>
    </RouterLink>
    <RouterLink :to="{ name: 'project' }" :class="route.name === 'project' ? 'active' : 'inactive'">
      <div class="flex h-8 items-center justify-center" data-icon="Table" data-size="24px" data-weight="regular">
        <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
          <path
            d="M224,48H32a8,8,0,0,0-8,8V192a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A8,8,0,0,0,224,48ZM40,112H80v32H40Zm56,0H216v32H96ZM216,64V96H40V64ZM40,160H80v32H40Zm176,32H96V160H216v32Z">
          </path>
        </svg>
      </div>
      <p>Projects</p>
    </RouterLink>
    <RouterLink :to="{ name: 'doc' }" :class="route.name === 'doc' ? 'active' : 'inactive'">
      <div class="flex h-8 items-center justify-center" data-icon="PersonSimpleWalk" data-size="24px"
        data-weight="regular">
        <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
          <path
            d="M152,80a32,32,0,1,0-32-32A32,32,0,0,0,152,80Zm0-48a16,16,0,1,1-16,16A16,16,0,0,1,152,32Zm64,112a8,8,0,0,1-8,8c-35.31,0-52.95-17.81-67.12-32.12-2.74-2.77-5.36-5.4-8-7.84l-13.43,30.88,37.2,26.57A8,8,0,0,1,160,176v56a8,8,0,0,1-16,0V180.12l-31.07-22.2L79.34,235.19A8,8,0,0,1,72,240a7.84,7.84,0,0,1-3.19-.67,8,8,0,0,1-4.15-10.52l54.08-124.37c-9.31-1.65-20.92,1.2-34.7,8.58a163.88,163.88,0,0,0-30.57,21.77,8,8,0,0,1-10.95-11.66c2.5-2.35,61.69-57.23,98.72-25.08,3.83,3.32,7.48,7,11,10.57C166.19,122.7,179.36,136,208,136A8,8,0,0,1,216,144Z">
          </path>
        </svg>
      </div>
      <p>doc</p>
    </RouterLink>
    <RouterLink :to="{ name: 'blog' }" :class="route.name === 'blog' ? 'active' : 'inactive'">
      <div class="flex h-8 items-center justify-center" data-icon="ChatTeardrop" data-size="24px" data-weight="regular">
        <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
          <path
            d="M132,24A100.11,100.11,0,0,0,32,124v84.33A15.69,15.69,0,0,0,47.67,224H132a100,100,0,0,0,0-200Zm0,184H48V124a84,84,0,1,1,84,84Z">
          </path>
        </svg>
      </div>
      <p>blog</p>
    </RouterLink>
  </footer>
  <div class="h-17 bg-[#34191a]"></div>
</template>