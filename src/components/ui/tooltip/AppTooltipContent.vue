<script setup lang="ts">
import { Tooltip } from '@vuetify/v0';
</script>

<template>
    <Tooltip.Content v-bind="$attrs" class="mf-menu-surface bg-transparent">
        <div class="rounded-lg px-2 py-1.5 text-[13px] bg-[#111c] text-white fill-white m-2 drop-shadow-[0_0_0.2em] drop-shadow-accent-500">
            <slot />
        </div>
    </Tooltip.Content>
</template>