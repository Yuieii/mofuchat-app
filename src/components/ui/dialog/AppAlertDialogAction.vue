<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Action v-bind="$attrs" class="grow px-5 py-3 align-center text-rose-600 transition duration-200 bg-rose-100 hover:bg-rose-200 active:bg-rose-300 cursor-pointer">
        <slot />
    </AlertDialog.Action>
</template>