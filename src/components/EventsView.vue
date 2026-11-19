<script setup lang="ts">
import { EventStore } from '../stores/events/index.ts';
import EventRenderer from './events/EventRenderer.vue';
</script>

<template>
    <div class="flex flex-col-reverse">
        <EventRenderer v-for="event in EventStore.events" :event />
    </div>
</template>