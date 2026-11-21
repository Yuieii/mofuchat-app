<script setup lang="ts">
import { computed, toRef } from 'vue';
import { ConfigStore } from '../../stores/config.ts';
import { EventStore, MofuComment, MofuEvent, MofuEventTypeLookup } from '../../stores/events';
import CommentRenderer from '../comments/CommentRenderer.vue';
import { AppTooltip } from '../ui/tooltip/index.ts';
import { FontAwesome } from '../ui/icons/index.ts';
import ContainerDebug from '../ContainerDebug.vue';

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

function getRepliedComment() {
    if (!comment.data.replyCommentId) return undefined;

    const reply = EventStore.events
        .filter(e => filterEvent(e, "commentCreated"))
        .map(e => e.data)
        .find(c => c.data.id === comment.data.replyCommentId);

    return reply;
}

function goToReply() {
    if (!replyComment.value) return;
    location.href = `#mofu-comment-${replyComment.value.data.id}`;
}

const replyComment = toRef(() => getRepliedComment());
</script>

<template>
    <div class="flex flex-col mx-2 my-1 text-accent-700 fill-accent-700">
        <!-- Reply container -->
        <div v-if="comment.data.replyCommentId" @click="goToReply()" class="z-1 shrink-0 mx-5 px-3 pb-[0.1em] gap-2 flex flex-row bg-[#f7ebf144] backdrop-blur-lg rounded-lg rounded-b-none h-8 border-1 border-b-0 border-accent-200 overflow-hidden items-center transition duration-200 hover:scale-[1.01] cursor-pointer">
            <div class="shrink-0">
                <FontAwesome.Reply class="h-4" />
            </div>
            <div v-if="replyComment" class="grow flex flex-row gap-2 min-w-0 mask-linear-90 mask-linear-from-[calc(100%_-_2em)] mask-linear-to-[100%]">
                <div class="shrink-0 flex flex-row items-center gap-2">
                    <div class="shrink-0"><img :src="replyComment.data.profileImage" class="w-4.5 aspect-square rounded-full" alt="" /></div>
                    <div class="flex flex-row items-end gap-1">
                        <span class="shrink-0">{{ replyComment.data.displayName ?? replyComment.data.name }}</span>
                        <span v-if="replyComment.data.displayName" class="text-xs opacity-50 shrink-0 before:content-['('] after:content-[')']">{{ replyComment.data.name }}</span>
                    </div>
                </div>
                <div class="grow min-w-0 relative flex flex-col">
                    <div class="w-[1000vw] align-middle -mt-[2px]">
                        <CommentRenderer v-for="renderer in replyComment.data.renderers" :renderer />
                    </div>
                </div>
            </div>
            <div v-else class="grow flex flex-row min-w-0">
                <span class="shrink-0">無法取得原訊息</span>
            </div>
        </div>

        <!-- Comment container -->
        <div :id="'mofu-comment-' + comment.data.id" class="z-2 shrink-0 flex flex-row bg-accent-60 rounded-l-lg rounded-xl items-stretch border-1 border-accent-200 overflow-hidden">
            <div class="shrink-0 w-1.5" :style="{ background: serviceCssColor() }"></div>

            <div class="grow flex flex-row gap-3 items-start px-3 py-2">
                <div class="shrink-0 py-1">
                    <img :src="comment.data.profileImage" class="w-10 aspect-square rounded-full" alt="" />
                </div>
                
                <div class="grow relative flex flex-col">
                    <div class="flex flex-row items-center gap-1">
                        <div class="flex flex-row shrink-0 items-baseline gap-1">
                            <span class="shrink-0 select-text">{{ comment.data.displayName ?? comment.data.name }}</span>
                            <span v-if="comment.data.displayName" class="select-text text-xs opacity-50 shrink-0 before:content-['('] after:content-[')']">{{ comment.data.name }}</span>
                        </div>
                        <div class="inline-flex shrink-0" v-for="badge in comment.data.badges">
                            <img class="w-4 aspect-square rounded-sm" :src="badge.url" :alt="badge.label" />
                        </div>
                    </div>
                    <div class="mr-1">
                        <CommentRenderer v-for="renderer in comment.data.renderers" class="select-text" :renderer />
                    </div>

                    <div class="absolute -top-1 -right-1 flex flex-col align-end group">
                        <div class="bg-accent-50 bg-accent-alpha-10 backdrop-blur-lg rounded-full py-1 px-1 divide-x-1 text-[11px] text-right flex flex-row group-hover:opacity-0 group-hover:h-0 group-hover:-z-10">
                            <span class="px-1.5 border-accent-300 shrink-0 opacity-50">{{ serviceName }}</span>
                            <span class="pl-1.5 border-accent-300 shrink-0 opacity-50">{{ getTimeString(comment.data.timestamp) }}</span>
                        </div>
                        <div class="-my-1.5 text-[13px] flex-row hidden group-hover:flex justify-end">
                            <div class="divide-x-1 shrink-0 backdrop-blur-lg leading-[1] rounded-full text-sm outline-none overflow-hidden">
                                <AppTooltip.Root class="border-[#a86d8133]" position-area="bottom">
                                    <AppTooltip.Activator>
                                        <button 
                                            class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 bg-accent-200 bg-accent-alpha-50 enabled:hover:bg-accent-250 enabled:hover:bg-accent-alpha-70 enabled:active:bg-accent-300 enabled:active:bg-accent-alpha-70 transition duration-200">
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
                                            class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 bg-accent-200 bg-accent-alpha-50 enabled:hover:bg-accent-250 enabled:hover:bg-accent-alpha-70 enabled:active:bg-accent-300 enabled:active:bg-accent-alpha-70 transition duration-200">
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
                                            class="px-[0.9em] py-[0.4em] fill-accent-700 disabled:opacity-50 bg-accent-200 bg-accent-alpha-50 enabled:hover:bg-accent-250 enabled:hover:bg-accent-alpha-70 enabled:active:bg-accent-300 enabled:active:bg-accent-alpha-70 transition duration-200"
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
    </div>
</template>
