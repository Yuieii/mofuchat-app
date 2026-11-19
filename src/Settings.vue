<script setup lang="ts">
import { ref, toRef } from "vue";
import "./assets/scss/style.scss";

import Button from "./components/Button.vue";
import ContainerDebug from "./components/ContainerDebug.vue";
import TitleBar from "./components/TitleBar.vue";
import { Select } from "@vuetify/v0";
import { AppSelect } from "./components/ui/select/index.ts";
import Slider from "./components/ui/widgets/Slider.vue";
import SettingsGroup from "./components/ui/settings/SettingsGroup.vue";
import SettingsItem from "./components/ui/settings/SettingsItem.vue";
import { FontAwesome } from "./components/ui/icons/index.ts";
import Switch from "./components/ui/widgets/Switch.vue";

const tabs = [
    { icon: "", label: "朗讀" },
    { icon: "", label: "留言" },
    { icon: "", label: "站點" },
    { icon: "", label: "UI" },
    { icon: "", label: "通知" },
    { icon: "", label: "API" },
    { icon: "", label: "其他" },
];

const tabIndex = ref(0);

const voices = speechSynthesis.getVoices();
console.log(voices);

const inactiveTabClasses = "rounded-lg px-3 py-2 text-accent-600 transition duration-100 hover:scale-[1.02] hover:bg-accent-100 active:scale-[1] active:bg-accent-200";
const activeTabClasses = "bg-accent-600 text-white rounded-lg px-3 py-2 text-accent-600 transition duration-100 hover:scale-[1.02] hover:bg-accent-600 active:scale-[1] active:bg-accent-700 hover:text-white";
</script>

<template>
    <div class="flex flex-col min-h-0 min-w-0 h-screen max-h-screen overscroll-none overflow-hidden bg-[#fff8]">
        <TitleBar>
            <template #left>
                <div class="absolute left-0 top-0 w-[33vw] h-full border-r-1 border-accent-500">
                </div>
            </template>
        </TitleBar>

        <div class="shrink-0 flex flex-row items-center bg-accent-600 text-white font-black">
            <div data-tauri-drag-region class="flex flex-row px-3 pb-2 items-stretch w-[33vw] border-r-1 border-accent-500">
                <div class="flex flex-row gap-2">
                    <h1 class="text-2xl px-3">設定</h1>
                </div>
            </div>

            <div data-tauri-drag-region class="grow flex flex-row px-3 pb-2">
                <div class="flex flex-row gap-2">
                    <h1 class="text-2xl px-3">{{ tabs[tabIndex].label }}</h1>
                </div>
            </div>
        </div>

        <main class="flex flex-row grow items-stretch basis-full min-h-0 min-w-0 max-w-screen">
            <div class="flex flex-col h-full max-h-full overflow-x-hidden overflow-y-auto w-[33vw] border-r-1 border-gray-300">
                <div class="flex flex-col gap-1 p-3">
                    <div v-for="(tab, index) in tabs" 
                        :class="index == tabIndex ? activeTabClasses : inactiveTabClasses"
                        @click="tabIndex = index">
                        {{ tab.label }}
                    </div>
                </div>
            </div>
            <div class="grow flex flex-col h-full items-stretch max-h-full overflow-x-hidden overflow-y-auto w-[33vw]">
                <div v-if="tabIndex == 0" class="flex flex-col p-3 max-w-[50em] gap-3">
                    <SettingsGroup>
                        <SettingsItem label="朗讀聲音">
                            <AppSelect.Root class="grow" v-model="voices[0]" :mandatory="true" :multiple="false">
                                <AppSelect.Activator>
                                    <AppSelect.Value v-slot="{ selectedValue }">
                                        {{ `${selectedValue.name} (${selectedValue.lang})` }}
                                    </AppSelect.Value>
                                    <AppSelect.Placeholder />
                                    <AppSelect.Cue />
                                </AppSelect.Activator>

                                <AppSelect.Content>
                                    <AppSelect.Item 
                                        v-for="voice in voices"
                                        :value="voice">
                                        {{ `${voice.name} (${voice.lang})` }}
                                    </AppSelect.Item>
                                </AppSelect.Content>
                            </AppSelect.Root>
                        </SettingsItem>

                        <SettingsItem label="朗讀音量">
                            <Slider class="grow" />
                        </SettingsItem>
                        <SettingsItem label="朗讀速度">
                            <Slider class="grow" />
                        </SettingsItem>
                        <SettingsItem label="朗讀音高">
                            <Slider class="grow" />
                        </SettingsItem>
                    </SettingsGroup>

                    <SettingsGroup>
                        <SettingsItem label="朗讀音高">
                            <Switch />
                        </SettingsItem>
                    </SettingsGroup>
                </div>
            </div>
        </main>
    </div>
</template>