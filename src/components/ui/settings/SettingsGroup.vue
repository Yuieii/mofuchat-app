<script setup lang="ts">
const { label, footer } = defineProps<{
    label?: string,
    footer?: string,
}>();
</script>

<template>
    <div class="flex flex-col">
        <span v-if="label" class="text-accent-600 px-5 py-1.5 text-sm font-black">{{ label }}</span>
        <div class="flex flex-col bg-accent-100 rounded-xl divide-y-1 border border-accent-200">
            <slot />
        </div>
        <span v-if="footer" class="text-accent-600 px-5 py-1.5 text-sm font-black">{{ footer }}</span>
    </div>
</template>