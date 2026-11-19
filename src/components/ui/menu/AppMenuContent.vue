<script setup lang="ts">
import { Menu } from '@vuetify/v0';
import { menuContentCommonStyles } from './styles';
</script>

<template>
    <Menu.Content v-bind="$attrs" :class="menuContentCommonStyles">
        <slot />
    </Menu.Content>
</template>