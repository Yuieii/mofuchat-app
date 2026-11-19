<script setup lang="ts">
import "./assets/scss/style.scss";

import Button from "./components/Button.vue";
</script>

<template>
    <div class="flex flex-col min-h-0 min-w-0 max-h-screen overscroll-none overflow-hidden bg-[#fff8]">
        <header data-tauri-drag-region class="shrink-0 flex flex-col items-stretch bg-accent-600 text-white font-black text-[95%]">
            <!-- Title bar-->

            <div data-tauri-drag-region class="flex flex-row h-[35px] left-[calc(50vw_-_4em)] items-center justify-center"> 
                <!-- Title bar center, floating on top -->
                <div data-tauri-drag-region class="flex flex-row w-screen items-center justify-center grow">
                    <span data-tauri-drag-region class="opacity-50 text-[90%]">MofuChat</span>
                </div>
            </div>

            <div data-tauri-drag-region class="flex flex-row items-center justify-between">
                <div class="flex flex-row px-3 pt-1 pb-2 items-stretch">
                    <!-- Title bar left side -->
                    <div class="hidden flex flex-row items-stretch px-5 bg-green-500">
                        <div class="w-[55px] h-full bg-red-500"></div>
                    </div>

                    <div class="flex flex-row gap-2">
                        <h1 class="text-4xl px-3 py-1">設定</h1>
                    </div>
                </div>

                <div class="flex flex-row px-3 pt-1 pb-2">
                    <!-- Title bar right side -->
                    <div class="flex flex-row gap-2">
                        
                    </div>
                </div>
            </div>
        </header>

        <main class="flex flex-col grow basis-full min-h-0 min-w-0 max-w-screen">
        </main>
    </div>
</template>