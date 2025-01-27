<script setup lang="ts">
defineProps({
  className: {
    type: String,
    default: '',
  },
  text: {
    type: String,
    default: '',
  },
});
</script>

<template>
  <h1 class="font-bold p-4 text-white bg-red-600 text-center" :class="className">
    🚧🚧 WARNING : {{ text }} 🚧🚧
  </h1>
</template>