<script setup lang="ts">
const props = defineProps({
  className: {
    type: String,
    default: '',
  },
});

</script>


<template>
  <footer :class="props.className">
    <p>
      <span>Since 2000 - {{ new Date().getFullYear() }} </span>
      <span> Made with love and too much coffee </span>
      <a href="https://github.com/batleforc">@Batleforc</a>
    </p>
  </footer>
</template>