<script setup lang="ts">
import { Tooltip } from '@vuetify/v0';
import type { TooltipRootProps } from "@vuetify/v0";

const { positionArea } = defineProps<TooltipRootProps>();
</script>

<template>
    <div class="inline-flex">
        <Tooltip.Root :open-delay="0" :close-delay="100" :position-area="positionArea ?? 'bottom'">
            <slot />
        </Tooltip.Root>
    </div>
</template>