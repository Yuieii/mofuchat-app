<script setup lang="ts">
import { Splitter } from '@vuetify/v0';
import { provide, ref } from 'vue';
import { Contexts } from '../../../context';

const currentOrientation = ref<"horizontal" | "vertical">("vertical");
provide(Contexts.splitterOrientation, currentOrientation);
</script>

<template>
    <Splitter.Root class="w-full h-full max-w-full max-h-full" v-bind="$attrs" v-slot="{ orientation }">
        {{(() => { currentOrientation = orientation; })() }}
        <slot />
    </Splitter.Root>
</template>