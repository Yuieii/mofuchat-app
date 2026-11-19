<script setup lang="ts">
import { onBeforeUnmount, ref, toRef, useTemplateRef, watch } from 'vue';
import ServiceItem from './ServiceItem.vue';
import { isNull, useDragDrop } from '@vuetify/v0';
import { ConfigStore } from '../stores/config.ts';
import type { Service } from '../stores/config.ts';

type Item = Service;
type Kinds = { type: "item", value: Item };

const config = ConfigStore;

const wrap = useTemplateRef<HTMLElement>("wrap");
const el = useTemplateRef<HTMLElement>("el");
const dnd = useDragDrop<Kinds>();
const zone = dnd.zones.register({
    el,
    accept: ["item"],
    orientation: "vertical",

    onDrop(drag, position) {
        if (config.data == null) return;

        const services = config.data.services;
        const fromIndex = services.findIndex(c => c.id === drag.value.id);
        if (fromIndex === -1) return;

        const toIndex = position.index ?? 0;
        const item = services[fromIndex];

        services.splice(fromIndex, 1);
        services.splice(toIndex + (fromIndex < toIndex ? -1 : 0), 0, item);
        config.saveConfig();
    }
});

const indicatorStyle = toRef(() => { 
    const ind = zone.indicator.value;
    const element = wrap.value;
    if (isNull(ind) || isNull(element)) return undefined;

    const rect = element.getBoundingClientRect();
    const y = ind.edge === "before" ? ind.rect.top : ind.rect.bottom;

    return {
        top: `${y}px`,
        left: `${ind.rect.left}px`,
        width: `${ind.rect.width}px`,
    };
});

onBeforeUnmount(() => zone.unregister());
</script>

<style lang="scss" scoped>
.flip-list-move {
    transition: transform 0.2s ease;
}

.flip-list-enter-from, .flip-list-leave-to {
    opacity: 0;
    height: 0;
    transform: translateX(-6px);
}

.flip-list-enter-to, .flip-list-leave-from {
    height: fit-content;
}

.flip-list-enter-active, .flip-list-leave-active {
    overflow: hidden;
    transition: all 0.2s ease;
}
</style>

<template>
    <div class="flex flex-col max-h-full min-h-0 overflow-y-auto">
        <div ref="wrap" class="grid grid-cols-[repeat(2,6fr)_repeat(5,auto)]">
            <div class="divide-x-1 grid grid-cols-subgrid col-span-7 sticky top-0 bg-[#fff3] backdrop-blur-xl z-1 border-b-1 border-b-[#0003]">
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">聊天室名稱</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">直播網址</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">朗讀</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">觀眾記錄</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">輸出</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">連接</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50"></div>
            </div>

            <div
                ref="el"
                class="grid grid-cols-subgrid col-span-7 divide-y-1 gap-y-1 p-1 bg-[#fff8]">
                <TransitionGroup type="transition" name="flip-list">
                    <ServiceItem v-for="i in config.data?.services" :key="i.id" :dnd :item="i" />
                </TransitionGroup>
            </div>

            <div v-if="zone.indicator.value" class="absolute z-10 h-1 transition-[top] rounded-full bg-accent-500" :style="indicatorStyle"></div>
        </div>
    </div>
</template>