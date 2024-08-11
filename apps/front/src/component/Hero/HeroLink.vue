<script setup lang="ts">
import IcoOrMedia from '../helper/IcoOrMedia.vue';
defineProps({
    icon: {
        type: String,
        required: true,
    },
    className: {
        type: String,
        default: '',
    },
    link: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <a :href="link.includes('/') ? link : `#${link}`" :target="link.includes('/') ? '_blank' : '_self'"
        rel="noreferrer">
        <IcoOrMedia :media="icon" :className="className" />
    </a>
</template>