<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Root v-bind="$attrs">
        <slot />
    </AlertDialog.Root>
</template>