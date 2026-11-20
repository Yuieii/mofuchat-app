<script setup lang="ts">
import { Select } from '@vuetify/v0';

</script>

<template>
    <Select.Activator 
        v-bind="$attrs" 
        class="text-accent-700 flex flex-row items-center justify-between w-full min-w-0 max-w-full rounded-lg border border-accent-500 bg-white text-sm leading-[1]">
    
        <slot />

    </Select.Activator>
</template>