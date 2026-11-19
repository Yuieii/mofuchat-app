<script setup lang="ts">
import { Select } from '@vuetify/v0';
import { FontAwesome } from '../icons';

</script>

<template>
    <Select.Cue v-bind="$attrs" v-slot="{ isOpen }">
        <FontAwesome.ChevronDown 
            class="transition fill-accent-500 h-4 duration-200"
            :class="[ isOpen ? 'rotate-180' : '' ]" />
    </Select.Cue>
</template>