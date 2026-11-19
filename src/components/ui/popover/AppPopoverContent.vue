<script setup lang="ts">
import { Popover } from '@vuetify/v0';
import { popoverCommonStyles } from '../menu/styles';

const classes = [...popoverCommonStyles];
</script>

<template>
    <Popover.Content :class="classes" v-bind="$attrs">
        <slot />
    </Popover.Content>
</template>