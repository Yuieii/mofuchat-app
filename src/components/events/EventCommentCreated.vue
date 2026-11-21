<script setup lang="ts">
import { computed } from 'vue';
import { ConfigStore } from '../../stores/config.ts';
import { EventStore, MofuComment, MofuEvent, MofuEventTypeLookup } from '../../stores/events';
import CommentRenderer from '../comments/CommentRenderer.vue';
import { AppTooltip } from '../ui/tooltip/index.ts';
import { FontAwesome } from '../ui/icons/index.ts';

const { comment } = defineProps<{
    comment: MofuComment
}>();

function serviceCssColor() {
    const color = comment.color;
    return `rgb(${color.r} ${color.g} ${color.b})`;
}

function getDateString(timestamp: number) {
    return new Date(timestamp).toLocaleString("zh-tw", {
        dateStyle: "short",
    });
}

const service = ConfigStore.data?.services.find(s => s.id == comment.id);

function getServiceName() {
    return service?.name ?? "未知服務";
}

const serviceName = computed(() => getServiceName());

function getTimeString(timestamp: string) {
    return new Date(timestamp).toLocaleString("en", {
        timeStyle: "medium",
    });
}

// @ts-ignore
function filterEvent<E extends MofuEvent, K extends keyof MofuEventTypeLookup>(event: E, type: K): event is MofuEvent<K> {
    return event.event === type;
}

function removeComment() {
    const index = EventStore.events.findIndex(e => {
        if (!filterEvent(e, "commentCreated")) return false;
        return e.data.data.id === comment.data.id;
    });
    EventStore.events.splice(index, 1);
}
</script>

<template>
    <div class="flex flex-row bg-[#f7ebf1] rounded-l-lg rounded-xl mx-2 my-1 items-stretch text-accent-700 border-1 border-accent-200 overflow-hidden">
        <div class="shrink-0 w-1.5" :style="{ background: serviceCssColor() }"></div>

        <div class="grow flex flex-row gap-3 items-start px-3 py-2">
            <div class="shrink-0 py-1">
                <img :src="comment.data.profileImage" class="w-10 aspect-square rounded-full" alt="" />
            </div>
            
            <div class="grow relative flex flex-col">
                <div class="flex flex-row items-center gap-1">
                    <span>{{ comment.data.displayName ?? comment.data.name }}</span>
                    <span v-if="comment.data.displayName" class="text-xs opacity-50">({{ comment.data.name }})</span>
                    <div class="inline-flex" v-for="badge in comment.data.badges">
                        <img class="w-4 aspect-square rounded-sm" :src="badge.url" :alt="badge.label" />
                    </div>
                </div>
                <div>
                    <CommentRenderer v-for="renderer in comment.data.renderers" :renderer />
                </div>

                <div class="absolute -top-1 -right-1 flex flex-col align-end group">
                    <div class="bg-[#f7ebf199] backdrop-blur-lg rounded-full py-1 px-1 divide-x-1 text-[11px] text-right flex flex-row group-hover:opacity-0 group-hover:h-0 group-hover:-z-10">
                        <span class="px-1.5 border-accent-300 shrink-0 opacity-50">{{ serviceName }}</span>
                        <span class="pl-1.5 border-accent-300 shrink-0 opacity-50">{{ getTimeString(comment.data.timestamp) }}</span>
                    </div>
                    <div class="-my-1.5 text-[13px] flex-row hidden group-hover:flex justify-end">
                        <div class="divide-x-1 shrink-0 bg-[#a86d8122] backdrop-blur-lg leading-[1] rounded-full text-sm outline-none overflow-hidden">
                            <AppTooltip.Root class="border-[#a86d8133]" position-area="bottom">
                                <AppTooltip.Activator>
                                    <button 
                                        class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 enabled:hover:bg-[#a86d8144] enabled:active:bg-[#a86d8166] transition duration-200">
                                        <FontAwesome.Ban class="h-4" />
                                    </button>
                                </AppTooltip.Activator>
                                <AppTooltip.Content>
                                    <span>封鎖</span>
                                </AppTooltip.Content>
                            </AppTooltip.Root>
                            <AppTooltip.Root class="border-[#a86d8133]" position-area="bottom">
                                <AppTooltip.Activator>
                                    <button 
                                        class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 enabled:hover:bg-[#a86d8144] enabled:active:bg-[#a86d8166] transition duration-200">
                                        <FontAwesome.Copy class="h-4" />
                                    </button>
                                </AppTooltip.Activator>
                                <AppTooltip.Content>
                                    <span>複製留言</span>
                                </AppTooltip.Content>
                            </AppTooltip.Root>
                            <AppTooltip.Root class="border-[#a86d8133]" position-area="bottom">
                                <AppTooltip.Activator>
                                    <button 
                                        class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 enabled:hover:bg-[#a86d8144] enabled:active:bg-[#a86d8166] transition duration-200"
                                        @click="removeComment()">
                                        <FontAwesome.Trash class="h-4" />
                                    </button>
                                </AppTooltip.Activator>
                                <AppTooltip.Content>
                                    <span>移除</span>
                                </AppTooltip.Content>
                            </AppTooltip.Root>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
