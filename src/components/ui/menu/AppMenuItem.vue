<script setup lang="ts">
import { Menu } from '@vuetify/v0';
import { menuItemCommonStyles } from './styles';

const { itemStyle } = defineProps<{
    itemStyle?: "normal" | "destructive"
}>();

function getClasses() {
    const defaults = [...menuItemCommonStyles];

    switch (itemStyle ?? "normal") {
        case "destructive":
            defaults.push("text-rose-400", "fill-rose-400");
            break;

        default:
            defaults.push("text-accent-500", "fill-accent-500");
            break;
    }

    return defaults.join(" ");
}
</script>

<template>
    <Menu.Item :class="getClasses()" v-bind="$attrs">
        <slot />
    </Menu.Item>
</template>