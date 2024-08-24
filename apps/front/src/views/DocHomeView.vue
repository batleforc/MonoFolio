<script setup lang="ts">
import { useDocStore } from '../stores/doc';

const docStore = useDocStore();
if (!docStore.inited && !docStore.docLoading) {
    docStore.init();
}
</script>

<template>
    <div id="title" class="container">
        <h1>Doc home page</h1>
    </div>
</template>