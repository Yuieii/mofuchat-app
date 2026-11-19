<script setup lang="ts">
import { Select } from '@vuetify/v0';

</script>

<template>
    <Select.Activator 
        v-bind="$attrs" 
        class="text-accent-700 flex items-center justify-between w-full px-3 py-2 rounded-lg border border-accent-500 bg-white">

        <slot />
    
    </Select.Activator>
</template>