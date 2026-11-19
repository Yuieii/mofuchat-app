<script setup lang="ts">
import { Slider } from '@vuetify/v0';

const emit = defineEmits<{
    (e: "commit"): void;
}>();

const {
    min = 0, max = 100, step = 1,
    gradient = "linear-gradient(to right, #000, #fff)",
    thumbColor = "#fff"
} = defineProps<{
    min?: number,
    max?: number,
    step?: number,
    gradient?: string,
    thumbColor?: string,
}>();

const model = defineModel<number>();
</script>

<template>
    <Slider.Root v-model="model" :min :max :step @end="emit('commit')" class="relative flex items-center mx-3 h-6" v-bind="$attrs">
        <Slider.Track 
            class="relative h-3 w-full rounded-full overflow-hidden" 
            :style="{ background: gradient }" />
        <Slider.Thumb 
            class="absolute top-1/2 size-5 rounded-full border-2 border-white drop-shadow-[0_0_0.1em] drop-shadow-gray-300 -translate-x-1/2 -translate-y-1/2" 
            :style="{ background: thumbColor }"/>
    </Slider.Root>
</template>