<script setup lang="ts">
import { Menu } from '@vuetify/v0';
</script>

<template>
    <Menu.Root v-bind="$attrs">
        <slot />
    </Menu.Root>
</template>