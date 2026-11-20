<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Cancel v-bind="$attrs" class="grow px-5 py-3 align-center text-accent-700 transition duration-200 hover:bg-[#0001] active:bg-#[0002] cursor-pointer">
        <slot />
    </AlertDialog.Cancel>
</template>