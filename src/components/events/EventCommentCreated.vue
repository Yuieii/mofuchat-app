<script setup lang="ts">
import { MofuComment } from '../../stores/events';

const { comment } = defineProps<{
    comment: MofuComment
}>();

function serviceCssColor() {
    const color = comment.color;
    return `rgb(${color.r} ${color.g} ${color.b})`;
}
</script>

<template>
    <div class="flex flex-row bg-gray-100 rounded-xl m-2 px-4 py-3 gap-4 items-start border-l-8 text-accent-700 border-1 border-gray-100" :style="{ borderLeftColor: serviceCssColor() }">
        <div>
            <img :src="comment.authorAvatar" class="w-15 aspect-square rounded-full" alt="" />
        </div>
        <div class="flex flex-col">
            <span>{{ comment.authorDisplayName }}</span>
            <span>{{ comment.renderers }}</span>
        </div>
    </div>
</template>
