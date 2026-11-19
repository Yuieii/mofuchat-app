<script setup lang="ts">
import type { Component } from 'vue';
import type { IconVariant } from '../icons';

const { icon, label, variant } = defineProps<{
    icon?: Component,
    variant?: IconVariant,
    label: string
}>();
</script>

<template>
    <div class="flex flex-row px-5 py-3 gap-1 border-accent-200 items-center justify-between">
        <div class="flex flex-row items-center gap-2 min-w-[20em] text-accent-600 fill-accent-600">
            <component v-if="icon" :is="icon" class="h-6" :variant />
            <slot name="icon" />
            <span>{{ label }}</span>
        </div>
        <slot />
    </div>
</template>