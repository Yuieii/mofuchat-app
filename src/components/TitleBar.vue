<script setup lang="ts">
import { FontAwesome } from './ui/icons/index.ts';
import { platform } from "@tauri-apps/plugin-os";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { PRODUCT_NAME } from '../constants.ts';

const { title } = defineProps<{
    title?: string
}>();

const currentPlatform = platform();

const titleBarHeight = (() => { 
    const isMain = getCurrentWindow().label === "main";

    switch (currentPlatform) {
        case "macos":
            return isMain ? "48px" : "35px";
        default:
            return isMain ? "40px" : "30px";
    }
})();
</script>

<template>
    <header data-tauri-drag-region class="shrink-0 relative flex flex-col items-stretch bg-accent-600 text-white font-black text-[95%]" :style="{ height: titleBarHeight }">
        <!-- Title bar-->

        <div data-tauri-drag-region class="flex flex-row h-full w-[8em] left-[calc(50vw_-_4em)] items-center justify-center absolute"> 
            <!-- Title bar center, floating on top -->
            <div data-tauri-drag-region class="flex flex-row w-screen items-center justify-center grow">
                <span data-tauri-drag-region class="opacity-50 text-[85%] leading-[1]">{{ title ?? PRODUCT_NAME }}</span>
            </div>
        </div>

        <div data-tauri-drag-region class="flex flex-row h-full items-center justify-between">
            <div data-tauri-drag-region class="flex flex-row px-3 self-stretch items-center gap-2">
                <!-- Title bar left side -->
                <div v-if="currentPlatform === 'macos'" data-tauri-drag-region class="opacity-0 flex flex-row items-stretch px-2 bg-green-500">
                    <div class="w-[55px] h-full bg-red-500"></div>
                </div>

                <div class="flex flex-row gap-2">
                    <slot name="left" />
                </div>
            </div>

            <div data-tauri-drag-region class="flex flex-row h-full self-stretch items-center">
                <!-- Title bar right side -->
                <div class="flex flex-row gap-2 px-3 py-0">
                    <slot name="right" />
                </div>

                <div v-if="currentPlatform !== 'macos'" data-tauri-drag-region class="flex flex-row self-stretch items-stretch">
                    <button class="z-1000 flex flex-row items-center justify-center w-14 hover:bg-accent-400 transition" @click="getCurrentWindow().close()">
                        <FontAwesome.Xmark class="h-5 fill-white" />
                    </button>
                </div>
            </div>
        </div>
    </header>
</template>