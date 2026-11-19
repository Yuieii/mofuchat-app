<script setup lang="ts">
import { Tooltip } from '@vuetify/v0';
import type { TooltipRootProps } from "@vuetify/v0";

const { positionArea } = defineProps<TooltipRootProps>();
</script>

<template>
    <Tooltip.Activator as="span" class="inline-flex" v-bind="$attrs">
        <slot />
    </Tooltip.Activator>
</template>