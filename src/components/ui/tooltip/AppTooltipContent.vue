<script setup lang="ts">
import { Tooltip } from '@vuetify/v0';
</script>

<template>
    <Tooltip.Content v-bind="$attrs" class="bg-transparent">
        <div class="rounded-lg p-3 bg-white text-accent-500 fill-accent-500 m-2 drop-shadow-[0_0_0.2em] drop-shadow-accent-500">
            <slot />
        </div>
    </Tooltip.Content>
</template>