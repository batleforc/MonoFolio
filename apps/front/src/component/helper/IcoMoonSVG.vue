<script setup lang="ts">
const props = defineProps({
  icon: {
    type: String,
    required: true,
  },
  className: {
    type: String,
    default: '',
  },
  fill: {
    type: String,
    default: '',
  }
});
let superClassName = `ico ico-${props.icon} ${props.className}`;
</script>

<template>
  <svg :class="superClassName" :fill="fill" focusable="false">
    <use :href="`/icon/symbol-defs.svg#ico-${icon}`"></use>
  </svg>
</template>