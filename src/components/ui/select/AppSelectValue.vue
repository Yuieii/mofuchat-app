<script setup lang="ts">
import { Select } from '@vuetify/v0';
</script>

<template>
    <Select.Value 
        v-bind="$attrs" 
        v-slot="slotProps"
        class="min-w-0 overflow-x-hidden overflow-y-hidden text-nowrap text-ellipsis px-3 py-2">
    
        <slot v-bind="slotProps ?? {}" />
        
    </Select.Value>
</template>