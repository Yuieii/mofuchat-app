<script setup lang="ts">
const { label } = defineProps<{
    label?: string
}>();
</script>

<template>
    <div class="flex flex-col">
        <span v-if="label" class="text-accent-600 px-5 py-2 font-black">{{ label }}</span>
        <div class="flex flex-col bg-accent-100 rounded-xl divide-y-1 border border-accent-200">
            <slot />
        </div>
    </div>
</template>