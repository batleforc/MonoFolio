<script setup lang="ts">
import { DocHeader } from '@portfolio/api-client';

defineProps<{
    metadata: DocHeader;
}>();
</script>

<template>
    <div class="pageHeader">
        <h1>{{ metadata.title }}</h1>
        <p>{{ metadata.date }}</p>
    </div>
</template>