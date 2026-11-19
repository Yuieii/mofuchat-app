<script setup lang="ts">
import { AlertDialog } from '@vuetify/v0';

</script>

<style lang="scss" scoped>
    .mf-dialog {
        --ue-transition-time: 0.2s;
        transform: scale(0.95) translate(0, 0);

        &::backdrop {
            background: var(--color-accent-800);
        }

        &, &::backdrop {
            opacity: 0;

            transition:
                opacity var(--ue-transition-time),
                transform var(--ue-transition-time),
                overlay var(--ue-transition-time) allow-discrete,
                display var(--ue-transition-time) allow-discrete;
        }
    }

    .mf-dialog:open {
        opacity: 1;
        transform: scale(1) translate(0, 0);

        &::backdrop {
            opacity: 0.3;
        }
    }

    @starting-style {
        .mf-dialog:open {
            opacity: 0;
            transform: scale(0.95) translate(0, 0);

            &::backdrop {
                opacity: 0;
            }
        }
    }
</style>

<template>
    <AlertDialog.Content class="mf-dialog open:flex flex-col m-auto bg-[#fffc] backdrop-blur-xl rounded-2xl drop-shadow-[0_0_1em] drop-shadow-accent-200" v-bind="$attrs">
        <slot />
    </AlertDialog.Content>
</template>