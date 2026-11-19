<script setup lang="ts">
import { h } from 'vue';
import type { VNode } from 'vue';
import { MofuEvent } from '../../stores/events';
import EventConnection from './EventConnection.vue';
import { ConfigStore, Service } from '../../stores/config.ts';
import EventCommentCreated from './EventCommentCreated.vue';

const { event } = defineProps<{
    event: MofuEvent
}>();

function getService(id: string): Service | undefined {
    return ConfigStore.data?.services.find(s => s.id == id);
}

function renderEvent(event: MofuEvent): VNode | null {
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

        default:
            return h(EventConnection, {
                class: "bg-gray-200! text-gray-700! fill-gray-700!"
            }, () => "未知事件。");
    }
}
</script>

<template>
    <template v-if="true">
        <component v-bind="$attrs" :is="() => renderEvent(event)" />
    </template>
</template>