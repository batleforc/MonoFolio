<script setup lang="ts">
import { Page } from '@portfolio/api-client';

const props = defineProps<{
    page: Page;
}>();
</script>

<template>
    <div>
        <h1>{{ page.name }}</h1>
        <p>{{ page.content }}</p>
    </div>
</template>