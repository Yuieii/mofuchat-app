<script setup lang="ts">
import { Select } from '@vuetify/v0';

</script>

<template>
    <Select.Item 
        v-bind="$attrs" 
        v-slot="{ isSelected, isHighlighted }">

        <div 
            class="px-3 py-2 rounded-md text-accent-700"
            :class="[
                isHighlighted ? 'bg-accent-600 text-white' :
                    isSelected ? '' : ''
            ]">
            <slot />
        </div>
    
    </Select.Item>
</template>