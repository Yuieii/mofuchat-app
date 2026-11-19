<script setup lang="ts">
</script>

<template>
    <button class="shrink-0 bg-accent-500 text-white leading-1 rounded-full px-[0.9em] py-[0.4em] text-sm transition duration-200 enabled:hover:scale-[1.05] enabled:hover:bg-accent-400 enabled:active:scale-[1] enabled:active:bg-accent-600 outline-none disabled:opacity-50">
        <span class="inline-flex items-center min-h-4"><slot /></span>
    </button>
</template>