<script setup lang="ts">
import { Menu } from '@vuetify/v0';
import { FontAwesome } from '../icons';
import { menuItemCommonStyles } from './styles';

const classes = [
    ...menuItemCommonStyles,
    "flex", "flex-row", "justify-between", "items-center",
    "text-accent-500", "fill-accent-500"
].join(" ");
</script>

<template>
    <Menu.SubActivator :class="classes">
        <slot />
        <FontAwesome.ChevronRight class="h-4 " />
    </Menu.SubActivator>
</template>