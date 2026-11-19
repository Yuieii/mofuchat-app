<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Cancel v-bind="$attrs" class="grow px-5 py-3 align-center text-accent-700">
        <slot />
    </AlertDialog.Cancel>
</template>