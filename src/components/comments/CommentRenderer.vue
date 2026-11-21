<script setup lang="ts">
import { h, VNode } from 'vue';
import { MofuCommentRenderer } from '../../stores/events';

const { renderer } = defineProps<{
    renderer: MofuCommentRenderer
}>();

const commonClasses = "align-middle";


function render(): VNode | null { 
    switch (renderer.type) {
        case "text":
            return h("span", {
                class: [commonClasses, ""].join(" ").trim()
            }, () => renderer.content);

        case "mention":
            return h("span", {
                class: [commonClasses, "inline-flex bg-accent-600 text-white pl-1.5 pr-2 py-[2px] leading-4 rounded-full before:content-['@'] text-[95%]"].join(" ").trim()
            }, () => renderer.content);

        case "emote":
            return h("img", {
                class: [commonClasses, "w-6 h-6 inline"].join(" ").trim(),
                src: renderer.url
            });

        default:
            return h("span", () => renderer);
    }
}

</script>

<template>
    <component :is="() => render()" />
</template>
