<script setup lang="ts">
import { ConfigStore } from '../../stores/config.ts';
import { MofuComment } from '../../stores/events';
import CommentRenderer from '../comments/CommentRenderer.vue';

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

function getServiceName() {
    return ConfigStore.data?.services.find(s => s.id == comment.id)?.name ?? "未知服務";
}

const serviceName = getServiceName();

function getTimeString(timestamp: number) {
    return new Date(timestamp).toLocaleString("en", {
        timeStyle: "medium",
    });
}
</script>

<template>
    <div class="flex flex-row bg-[#f7ebf1] rounded-xl mx-2 my-1 items-stretch text-accent-700 border-1 border-accent-200 overflow-hidden">
        <div class="w-1" :style="{ background: serviceCssColor() }"></div>

        <div class="grow flex flex-row gap-3 items-start px-3 py-2">
            <div class="py-1">
                <img :src="comment.data.profileImage" class="w-10 aspect-square rounded-full" alt="" />
            </div>
            <div class="grow relative flex flex-col">
                <div class="flex flex-row items-center gap-1">
                    <span>{{ comment.data.displayName }}</span>
                    <div class="inline-flex" v-for="badge in comment.data.badges">
                        <img class="w-4 aspect-square rounded-sm" :src="badge.url" :alt="badge.label" />
                    </div>
                </div>
                <div>
                    <CommentRenderer v-for="renderer in comment.data.renderers" :renderer />
                </div>

                <div class="divide-x-1 bg-[#f7ebf1] absolute top-0 right-0 opacity-50 text-[11px] text-right flex flex-row">
                    <span class="px-1.5 border-accent-300">{{ serviceName }}</span>
                    <span class="pl-1.5 border-accent-300">{{ getTimeString(comment.data.timestamp * 1000) }}</span>
                </div>
            </div>
        </div>
    </div>
</template>
