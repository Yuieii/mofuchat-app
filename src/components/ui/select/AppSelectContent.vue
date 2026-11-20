<script setup lang="ts">
import { Select } from '@vuetify/v0';

</script>

<template>
    <Select.Content 
        v-bind="$attrs" 
        class="mf-menu-surface p-1 rounded-lg border border-accent-500 bg-white max-h-[50vh] origin-top" :style="{ minWidth: 'anchor-size(width)' }">

        <slot />
    
    </Select.Content>
</template>