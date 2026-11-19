<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<template>
    <AlertDialog.Description class="block text-accent-700 min-w-80" v-bind="$attrs">
        <slot />
    </AlertDialog.Description>
</template>