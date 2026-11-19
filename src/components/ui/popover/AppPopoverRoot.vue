<script setup lang="ts">
import { Popover } from '@vuetify/v0';
</script>

<template>
    <Popover.Root v-bind="$attrs">
        <slot />
    </Popover.Root>
</template>