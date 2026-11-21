<script setup lang="ts">
import { Splitter } from '@vuetify/v0';
import { ConstructorOf } from '../../../types';
import { inject } from 'vue';
import { Contexts } from '../../../context';

const props = defineProps({
    orientation: String as unknown as ConstructorOf<"vertical" | "horizontal">
});

const contextOrientation = inject(Contexts.splitterOrientation);

function getClasses() {
    const defaults = "flex flex-col opacity-50 justify-center items-center p-[3px] bg-accent-200 group hover:bg-accent-300 active:bg-accent-400".split(" ");
    const orientation = props.orientation ?? contextOrientation?.value;

    switch (orientation) {
        case "vertical":
            defaults.push("-my-[4px]");
            break;

        case "horizontal":
            defaults.push("-mx-[4px]", "h-full");
            break;
    }

    return defaults.join(" ");
}

function getClassesInner() {
    const defaults = ["bg-accent-400", "group-hover:bg-white", "group-active:bg-white"];
    const orientation = props.orientation ?? contextOrientation?.value;

    switch (orientation) {
        case "vertical":
            defaults.push("w-10", "h-[2px]");
            break;

        case "horizontal":
            defaults.push("h-10", "w-[2px]");
            break;
    }

    return defaults.join(" ");    
}
</script>

<template>
    <Splitter.Handle class="z-1000 opacity-0" v-bind="$attrs">
        <div :class="getClasses()">
            <div :class="getClassesInner()"></div>
        </div>
    </Splitter.Handle>
</template>