<script setup lang="ts">
import { Splitter } from '@vuetify/v0';
</script>

<template>
    <Splitter.Panel v-bind="$attrs">
        <div class="flex w-full h-full max-w-full max-h-full overflow-auto">
            <div class="flex flex-row w-full align-stretch justify-stretch p-2">
                <div class="flex flex-col w-full align-stretch justify-stretch shadow-[0_0_1em] shadow-accent-200 rounded-lg overflow-hidden">
                    <slot />
                </div>
            </div>
        </div>
    </Splitter.Panel>
</template>