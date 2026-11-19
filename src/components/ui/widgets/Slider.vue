<script setup lang="ts">
import { Slider } from '@vuetify/v0';
</script>


<template>
    <Slider.Root class="relative flex items-center mx-3 h-4" v-bind="$attrs">
        <Slider.Track class="relative h-1 w-full rounded-full overflow-hidden bg-accent-200">
            <Slider.Range class="absolute h-1 bg-accent-500" />
        </Slider.Track>

        <Slider.Thumb class="absolute top-1/2 size-3 rounded-full bg-accent-500 -translate-x-1/2 -translate-y-1/2" />
    </Slider.Root>
</template>