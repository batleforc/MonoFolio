<script setup lang="ts">

</script>

<template>
    <div class="container">
        <h1 v-for="truc in [...Array(100).keys()]" :key="truc">Home {{ truc }}</h1>
    </div>
</template>