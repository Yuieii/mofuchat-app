<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Title class="block text-accent-700 text-lg font-normal" v-bind="$attrs">
        <slot />
    </AlertDialog.Title>
</template>