<script setup lang="ts">
import { Menu } from '@vuetify/v0';
import { menuContentCommonStyles } from './styles';
</script>

<template>
    <Menu.SubContent :class="menuContentCommonStyles" v-bind="$attrs">
        <slot />
    </Menu.SubContent>
</template>