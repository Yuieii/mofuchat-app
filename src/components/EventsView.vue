<script setup lang="ts">
import { Virtualizer } from '@vuetify/v0';
import EventRenderer from './events/EventRenderer.vue';
import { EventStore } from '../stores/events/index.ts';
</script>

<style lang="scss" scoped>
.mofu-event-enter-from, .mofu-event-leave-to {
    opacity: 0;
    transform: translateX(-60px);
}
.mofu-event-enter-to, .mofu-event-leave-from {
    opacity: 1;
    transform: none;
}
.mofu-event-enter-active, .mofu-event-leave-active {
    transition: transform 0.3s, opacity 0.3s;
    transition-timing-function: cubic-bezier(.06,.83,.42,.99);
}
</style>

<template>
    <div class="w-full h-full overflow-hidden">
        <Virtualizer.Root :style="{ height: '100%' }" :items="EventStore.events.toReversed()" v-slot="{ items }" direction="reverse">
            <Virtualizer.Spacer />
            
            <TransitionGroup name="mofu-event">
                <Virtualizer.Item
                    v-for="item in items"
                    :key="item.raw.key"
                    :index="item.index">
                    <EventRenderer :event="item.raw" />
                </Virtualizer.Item>
            </TransitionGroup>
            <Virtualizer.Spacer edge="end" />
        </Virtualizer.Root>
    </div>

    <!-- <div class="flex flex-col-reverse">
        <TransitionGroup name="mofu-event">
            <EventRenderer v-for="event in EventStore.events" :key="event.key" :event />
        </TransitionGroup>
    </div> -->
</template>