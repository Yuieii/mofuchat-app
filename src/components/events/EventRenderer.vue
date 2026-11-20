<script setup lang="ts">
import { h } from 'vue';
import type { VNode } from 'vue';
import { MofuEvent, MofuEventTypeLookup } from '../../stores/events';
import EventConnection from './EventConnection.vue';
import { ConfigStore, Service } from '../../stores/config.ts';
import EventCommentCreated from './EventCommentCreated.vue';
import EventRequireSignIn from './EventRequireSignIn.vue';

const { event } = defineProps<{
    event: MofuEvent
}>();

function getService(id: string): Service | undefined {
    return ConfigStore.data?.services.find(s => s.id == id);
}

function assumeKnownKey(event: MofuEvent): MofuEvent<keyof MofuEventTypeLookup> {
    // @ts-ignore;
    return event;
}

function renderEvent<K extends string>(ev: MofuEvent<K>): VNode | null {
    const event = assumeKnownKey(ev);
    switch (event.event) {
        case "serviceConnected":
            return h(EventConnection, {
                class: "bg-lime-200! text-lime-700! fill-lime-700!"
            }, () => `已連線：${getService(event.data.id)?.name}`);

        case "serviceDisconnected":
            return h(EventConnection, {
                class: "bg-orange-200! text-orange-700! fill-orange-700!"
            }, () => `已中斷連線：${getService(event.data.id)?.name}`);

        case "serviceDeactivated":
            return h(EventConnection, {
                class: "bg-rose-200! text-rose-700! fill-rose-700!"
            }, () => `已離線：${getService(event.data.id)?.name}`);

        case "commentCreated":
            return h(EventCommentCreated, {
                comment: event.data,
                class: "text-sm"
            });

        case "requiresSignIn":
            return h(EventRequireSignIn, {
                data: event.data
            });

        default:
            return h(EventConnection, {
                class: "bg-gray-200! text-gray-700! fill-gray-700! items-center gap-2"
            }, [
                h("span", "未知事件。"),
                h("code", { class: "opacity-50 text-xs font-monospace bg-[#0002] px-1 mt-0.5 pt-0.5 rounded-sm border border-[#0003]" }, `event: "${ev.event}"`)
            ]);
    }
}
</script>

<template>
    <template v-if="true">
        <component v-bind="$attrs" :is="() => renderEvent(event)" />
    </template>
</template>