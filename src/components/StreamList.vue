<script setup lang="ts">
import { ref } from 'vue';
import ServiceItem from './ServiceItem.vue';
import StreamItem from './StreamItem.vue';

const list = ref([0].map(c => { 
  return { n: c };
}));
</script>

<style lang="scss" scoped>
.flip-list-move {
    transition: transform 0.6s ease;
}
</style>

<template>

    <div class="flex flex-col max-h-full min-h-0 overflow-y-auto overscroll-contain">
        <div class="grid grid-cols-[repeat(2,auto)_repeat(1,minmax(auto,100vw))_repeat(5,auto)]">
            <div class="divide-x-1 grid grid-cols-subgrid col-span-8 sticky top-0 bg-[#fff3] backdrop-blur-xl z-1 border-b-1 border-b-[#0003]">
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">OnAir</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">聊天室名稱</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">標題</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">已登入</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">經過時間</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">觀眾人數</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">總觀眾數</div>
                <div class="border-[#0001] flex flex-row justify-center items-center py-2 px-3 text-sm text-accent-600 opacity-50">其他</div>
            </div>

            <div class="grid grid-cols-subgrid col-span-8 divide-y-1 gap-y-1 p-1 bg-[#fff8]">
                <TransitionGroup type="transition" name="flip-list">
                    <StreamItem v-for="i in list" :key="i.n" />
                </TransitionGroup>
            </div>

        </div>
    </div>
</template>