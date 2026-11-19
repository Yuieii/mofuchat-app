<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Action v-bind="$attrs" class="grow px-5 py-3 align-center text-rose-600 bg-rose-100">
        <slot />
    </AlertDialog.Action>
</template>