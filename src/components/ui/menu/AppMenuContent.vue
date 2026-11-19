<script setup lang="ts">
import { Menu } from '@vuetify/v0';
import { menuContentCommonStyles } from './styles';
</script>

<style lang="scss">
    .mf-menu-surface {
        --ue-transition-time: 0.2s;

        opacity: 0;
        transform: scale(0.95) translate(0, 0);

        transition:
            opacity var(--ue-transition-time),
            transform var(--ue-transition-time),
            overlay var(--ue-transition-time) allow-discrete,
            display var(--ue-transition-time) allow-discrete;
    }

    .mf-menu-surface:popover-open {
        opacity: 1;
        transform: scale(1) translate(0, 0);
    }

    @starting-style {
        .mf-menu-surface:popover-open {
            opacity: 0;
            transform: scale(0.95) translate(0, 0);
        }
    }
</style>

<template>
    <Menu.Content v-bind="$attrs" :class="menuContentCommonStyles">
        <slot />
    </Menu.Content>
</template>