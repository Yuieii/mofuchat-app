<script setup lang="ts">
import { EventStore } from '../stores/events/index.ts';
import EventRenderer from './events/EventRenderer.vue';
</script>

<style lang="scss" scoped>
.mofu-event-leave-to {
    opacity: 0;
}
.mofu-event-enter-from {
    transform: translateY(-60px);
}
.mofu-event-leave-to {
    transform: translateY(-60px);
}
.mofu-event-enter-to, .mofu-event-leave-from {
    opacity: 1;
    transform: none;
}
.mofu-event-move, .mofu-event-enter-active, .mofu-event-leave-active {
    transition: opacity 0.3s, transform 0.3s;
    transition-timing-function: cubic-bezier(.06,.83,.42,.99)
}
.mofu-event-leave-active {
    position: absolute;
    width: 100%;
}
</style>

<template>
    <div class="flex flex-col-reverse">
        <TransitionGroup name="mofu-event">
            <EventRenderer v-for="event in EventStore.events" :key="event.key" :event />
        </TransitionGroup>
    </div>
</template>