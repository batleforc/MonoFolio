<script setup lang="ts">

</script>

<template>
  <div id="title" class="container">
    <RouterLink :to="{ name: 'doc' }">
      <h1 class="blogHomeView">Blog home page is a WIP feature yet, if you search the doc click on me</h1>
    </RouterLink>
  </div>
</template>

<style lang="scss">
.blogHomeView {
  @apply text-4xl;
}
</style>