<script setup lang="ts">
import Menubar from 'primevue/menubar';
defineProps({
    link: {
        type: Array<{
            icon: string,
            label: string,
            to: string
        }>,
        required: false
    }
});
const items = [
    {
        icon: 'ico#home',
        label: 'Home',
        to: '/'
    },
    {
        icon: 'ico#doc',
        label: 'Doc',
        to: '/doc'
    },
    {
        icon: 'ico#blog',
        label: 'Blog',
        to: '/blog'
    }
]

</script>

<template>
    <Menubar class="sticky" :model="items" />
</template>