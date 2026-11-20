<script setup lang="ts">
import { Select, SelectItemProps } from '@vuetify/v0';

const { value } = defineProps<SelectItemProps>();
</script>

<template>
    <Select.Item 
        v-bind="$attrs" 
        v-slot="{ isSelected, isHighlighted }"
        :value>

        <div 
            class="px-3 py-2 rounded-md text-accent-700 text-sm leading-[1]"
            :class="[
                isHighlighted ? 'bg-accent-600 text-white' :
                    isSelected ? '' : ''
            ]">
            <slot />
        </div>
    
    </Select.Item>
</template>