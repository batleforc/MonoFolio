<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { useDocStore } from '../../stores/doc';
const props = defineProps<{
    technoName: string,
}>();

const docStore = useDocStore();

const techno = docStore.technoExists(props.technoName);

</script>

<template>
    <div>
        <RouterLink v-if="techno" :to="{ name: 'doccontent', params: { page: techno.path.split('/') } }">
            <Chip :label="props.technoName" />
        </RouterLink>
        <Chip v-else :label="props.technoName" />
    </div>
</template>