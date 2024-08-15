<script setup lang="ts">
import IcoOrMedia from '../helper/IcoOrMedia.vue';
const props = defineProps({
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
    smooth: {
        type: Boolean,
        default: true,
    },
    internal: {
        type: Boolean,
        default: true,
    },
    label: {
        type: String,
        required: false
    },
    labelClassName: {
        type: String,
        default: ''
    },
    linkClassName: {
        type: String,
        default: ''
    }
});
const smoothScroll = (event: MouseEvent) => {
    if (!event) return;
    event.preventDefault();
    document.querySelector(`#${props.link}`)?.scrollIntoView({ behavior: 'smooth' });
}
</script>

<template>
    <a v-if="!internal || smooth" :class="linkClassName"
        @click="(event) => smooth && !link.includes('/') && smoothScroll(event)"
        :href="link.includes('/') ? link : `#${link}`" target="_blank" rel="noreferrer">
        <IcoOrMedia :media="icon" :className="className" />
        <span v-if="label !== undefined" :class="labelClassName">{{ label }}</span>
    </a>
    <router-link v-else :to="link" :class="linkClassName">
        <IcoOrMedia :media="icon" :className="className" />
        <span v-if="label !== undefined" :class="labelClassName">{{ label }}</span>
    </router-link>
</template>