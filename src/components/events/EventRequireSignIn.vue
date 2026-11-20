<script setup lang="ts">
import { ref } from 'vue';
import { ConfigStore } from '../../stores/config.ts';
import { MofuComment, MofuEvent, MofuEventTypeLookup } from '../../stores/events';
import CommentRenderer from '../comments/CommentRenderer.vue';
import { openUrl } from "@tauri-apps/plugin-opener";
import { FontAwesome } from '../ui/icons/index.ts';
import { AppTooltip } from '../ui/tooltip/index.ts';

const { data } = defineProps<{
    data: MofuEventTypeLookup["requiresSignIn"]
}>();

function getServiceName() {
    switch (data.platform) {
        case "twitch":
            return "Twitch";

        default:
            return data.platform;
    }
}

const collapsed = ref(true);

async function openBrowser() {
    try {
        await openUrl(data.url);
    } catch (ex) {
        console.error(ex);
    }
}
</script>

<style lang="scss" scoped>

.collapsed-view-enter-from, .collapsed-view-leave-to {
    opacity: 0;
    padding-block: 0;
    height: 0;
}
.collapsed-view-enter-from {
    // transform: translateX(-60px);
}
.collapsed-view-leave-to {
    // transform: translateX(-60px);
}
.collapsed-view-enter-to, .collapsed-view-leave-from {
    opacity: 1;
    height: 64px;
    transform: none;
}
.collapsed-view-move, .collapsed-view-enter-active, .collapsed-view-leave-active {
    transition: all 0.3s;
    transition-timing-function: cubic-bezier(.06,.83,.42,.99)
}

</style>

<template>
    <div class="flex flex-col text-sm border border-accent-300 bg-accent-100 fill-accent-500 text-accent-500 m-1 rounded-xl overflow-hidden" @click="collapsed = !collapsed">
        <div class="flex flex-row items-center justify-between px-3 py-1.5">
            <span>需要登入：{{ getServiceName() }}</span>
            <FontAwesome.ChevronDown class="h-4 transition" :class="!collapsed ? 'rotate-180' : ''" />
        </div>

        <Transition name="collapsed-view">
            <div v-if="!collapsed" class="flex flex-col px-3 py-1.5 bg-[#fff8]">
                <template v-if="!data.code">
                    <span>
                        請點擊<AppTooltip.Root>
                            <AppTooltip.Activator>
                                <span @click="openBrowser()" class="underline cursor-pointer">這個連結</span>
                            </AppTooltip.Activator>
                            <AppTooltip.Content>
                                <span>{{ data.url }}</span>
                            </AppTooltip.Content>
                        </AppTooltip.Root>至登入網頁。
                    </span>
                </template>
                <template v-else>
                    <span>
                        請點擊<AppTooltip.Root>
                            <AppTooltip.Activator>
                                <span @click="openBrowser()" class="underline cursor-pointer">這個連結</span>
                            </AppTooltip.Activator>
                            <AppTooltip.Content>
                                <span>{{ data.url }}</span>
                            </AppTooltip.Content>
                        </AppTooltip.Root>至登入網頁，並輸入以下代碼：
                    </span>
                    <span class="select-text text-2xl">{{ data.code }}</span>
                </template>
            </div>
        </Transition>
    </div>
</template>
