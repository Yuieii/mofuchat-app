<script setup lang="ts">
import { computed, onMounted, ref, toRef, TransitionGroup, watch } from "vue";
import "./assets/scss/style.scss";

import Button from "./components/Button.vue";
import ContainerDebug from "./components/ContainerDebug.vue";
import TitleBar from "./components/TitleBar.vue";
import { createPermissionsPlugin, Select } from "@vuetify/v0";
import { AppSelect } from "./components/ui/select/index.ts";
import Slider from "./components/ui/widgets/Slider.vue";
import SettingsGroup from "./components/ui/settings/SettingsGroup.vue";
import SettingsItem from "./components/ui/settings/SettingsItem.vue";
import { FontAwesome } from "./components/ui/icons/index.ts";
import Switch from "./components/ui/widgets/Switch.vue";
import NumberInput from "./components/ui/widgets/NumberInput.vue";
import { platform, version } from "@tauri-apps/plugin-os";
import { ConfigStore, MofuConfig, useConfig } from "./stores/config.ts";
import SettingsPage from "./components/ui/settings/SettingsPage.vue";
import { useWindowStyling } from "./composables/index.ts";

const currentPlatform = platform();

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

speechSynthesis.addEventListener("voiceschanged", _ => {
    voices.value = speechSynthesis.getVoices();
});

const voices = ref<SpeechSynthesisVoice[]>(speechSynthesis.getVoices());

const limitSpeechCommentLength = ref(false);
const limitSpeechUserNameLength = ref(false);

const inactiveTabClasses = "rounded-lg px-3 py-2 text-accent-600 transition duration-100 hover:scale-[1.02] hover:bg-accent-100 active:scale-[1] active:bg-accent-200";
const activeTabClasses = "bg-accent-600 text-white rounded-lg px-3 py-2 text-accent-600 transition duration-100 hover:scale-[1.02] hover:bg-accent-600 active:scale-[1] active:bg-accent-700 hover:text-white";

interface VisualStyle {
    id: string,
    name: string,
    platform: string[]
};

const visualStyles: VisualStyle[] = [
    { id: "normal",   name: "一般",               platform: ["windows", "macos"] },
    { id: "mica",     name: "Windows 11 Mica",    platform: ["windows"] },
    { id: "acrylic",  name: "Windows 10 Acrylic", platform: ["windows"] },
    { id: "vibrancy", name: "macOS 毛玻璃背景",   platform: ["macos"] },
    { id: "glass",    name: "macOS Liquid Glass", platform: ["macos"], requirement: () => parseInt(version().split(".")[0]) >= 26 },
].filter(c => c.platform.includes(currentPlatform) && (!c.requirement || c.requirement()));

console.log(version());

const visualStyle = ref<VisualStyle>(visualStyles[0]);

const initialized = ref(false);

const maxWSize = "max-w-[1280px]";

const { defaultBackground } = useWindowStyling();

const { loaded } = useConfig();

watch(() => visualStyle.value, async s => { 
    if (!initialized.value) return;

    const data = ConfigStore.data;
    if (!data) return;

    // @ts-ignore
    data.config.mofu.windowStyle = s.id;
    await ConfigStore.saveConfig();
});

onMounted(async () => {
    initialized.value = false;
    await loaded;

    const config = ConfigStore.data!.config;

    // @ts-ignore
    visualStyle.value = visualStyles.find(c => c.id == config.mofu.windowStyle);
    initialized.value = true;
});
</script>

<style lang="scss" scoped>
.tab-content-enter-from, .tab-content-leave-to {
    opacity: 0;
    order: 2;
}
.tab-content-enter-from {
    transform: translateX(60px);
}
.tab-content-leave-to {
    transform: translateX(-60px);
}
.tab-content-enter-to, .tab-content-leave-from {
    opacity: 1;
    order: 1;
    transform: none;
}
.tab-content-move, .tab-content-enter-active, .tab-content-leave-active {
    transition: opacity 0.2s, transform 0.5s;
    transition-timing-function: cubic-bezier(.06,.83,.42,.99)
}
.tab-content-leave-active {
    position: absolute;
    width: 100%;
}
</style>

<template>
    <div class="flex flex-col min-h-0 min-w-0 h-screen max-h-screen overscroll-none overflow-hidden" :class="defaultBackground">
        <TitleBar>
            <template #left>
                <div data-tauri-drag-region class="absolute left-0 top-0 w-screen h-full">
                    <div data-tauri-drag-region class="@container-[inline-size] flex flex-row h-full mx-auto" :class="maxWSize">
                        <div data-tauri-drag-region class="border-r-1 border-accent-500 w-[33cqi] h-full"></div>
                    </div>
                </div>
            </template>
        </TitleBar>

        <div class="shrink-0 bg-accent-600 text-white font-black">

            <div class="@container-[inline-size] flex flex-row w-screen mx-auto" :class="maxWSize">
                <div data-tauri-drag-region class="flex flex-row px-3 pb-2 items-stretch w-[33cqi] border-r-1 border-accent-500">
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

        </div>

        <main class="@container-[inline-size] w-full flex flex-row grow items-stretch basis-full min-h-0 min-w-0 mx-auto" :class="maxWSize">
            <div class="flex flex-col h-full max-h-full overflow-x-hidden overflow-y-auto w-[33cqi] border-r-1 border-gray-300">
                <div class="flex flex-col gap-1 p-3">
                    <template v-for="(tab, index) in tabs" :key="index">
                        <div :class="index == tabIndex ? activeTabClasses : inactiveTabClasses" @click="tabIndex = index">
                            {{ tab.label }}
                        </div>
                    </template>
                </div>
            </div>
            <div class="grow relative flex flex-col h-full items-stretch max-h-full overflow-x-hidden overflow-y-auto scrollbar-gutter-stable">
                <TransitionGroup name="tab-content">
                    <SettingsPage v-if="tabIndex == 0" :key="0">
                        <SettingsGroup>
                            <SettingsItem label="朗讀聲音">
                                <AppSelect.Root v-model="voices[0]" :mandatory="true" :multiple="false">
                                    <AppSelect.Activator class="grow">
                                        <AppSelect.Value v-slot="{ selectedValue }">
                                            {{ `${(selectedValue as SpeechSynthesisVoice).name} (${(selectedValue as SpeechSynthesisVoice).lang})` }}
                                        </AppSelect.Value>
                                        <AppSelect.Placeholder />
                                        <AppSelect.Cue />
                                    </AppSelect.Activator>

                                    <AppSelect.Content>
                                        <AppSelect.Item v-for="voice in voices" :key="voice.voiceURI" :value="voice">
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
                            <SettingsItem label="限制朗讀留言字數">
                                <Switch v-model="limitSpeechCommentLength" />
                            </SettingsItem>
                            <SettingsItem v-if="limitSpeechCommentLength">
                                <input type="number" :min="0" :max="50" :value="50" />
                            </SettingsItem>
                        </SettingsGroup>

                        <SettingsGroup>
                            <SettingsItem label="限制朗讀留言字數">
                                <Switch v-model="limitSpeechUserNameLength" />
                            </SettingsItem>
                            <SettingsItem v-if="limitSpeechUserNameLength">
                                <input type="number" :min="0" :max="50" :value="50" />
                            </SettingsItem>
                        </SettingsGroup>
                    </SettingsPage>
                    
                    <SettingsPage v-if="tabIndex == 1" :key="1">
                        <SettingsGroup>
                            <SettingsItem label="記錄時使用暱稱">
                                <Switch />
                            </SettingsItem>
                            <SettingsItem label="將指定字元開頭的留言註冊為暱稱">
                                <Switch />
                            </SettingsItem>
                            <SettingsItem label="輸出免費禮物">
                                <Switch />
                            </SettingsItem>
                            <SettingsItem label="替換使用者頭像">
                                <Switch />
                            </SettingsItem>
                            <SettingsItem label="在應用程式中顯示使用者頭像">
                                <Switch :disabled="true" />
                            </SettingsItem>
                        </SettingsGroup>
                    </SettingsPage>
                    
                    <SettingsPage v-if="tabIndex == 3" :key="3">
                        <SettingsGroup>
                            <SettingsItem label="記錄時使用暱稱">
                                <Switch />
                            </SettingsItem>
                        </SettingsGroup>

                        <SettingsGroup>
                            <SettingsItem label="視窗樣式">
                                <AppSelect.Root v-model="visualStyle" :mandatory="true" :multiple="false">
                                    <AppSelect.Activator class="grow">
                                        <AppSelect.Value v-slot="{ selectedValue }">
                                            {{ `${(selectedValue as VisualStyle).name}` }}
                                        </AppSelect.Value>
                                        <AppSelect.Placeholder />
                                        <AppSelect.Cue />
                                    </AppSelect.Activator>

                                    <AppSelect.Content>
                                        <template v-for="visual in visualStyles" :key="visual.id">
                                            <AppSelect.Item :value="visual">
                                                {{ `${visual.name}` }}
                                            </AppSelect.Item>
                                        </template>
                                    </AppSelect.Content>
                                </AppSelect.Root>
                            </SettingsItem>
                        </SettingsGroup>

                        <SettingsGroup label="主題" footer="選擇一個喜歡的主題並套用。">
                            <SettingsItem label="視窗樣式">
                                <AppSelect.Root v-model="visualStyle" :mandatory="true" :multiple="false">
                                    <AppSelect.Activator class="grow">
                                        <AppSelect.Value v-slot="{ selectedValue }">
                                            {{ `${(selectedValue as VisualStyle).name}` }}
                                        </AppSelect.Value>
                                        <AppSelect.Placeholder />
                                        <AppSelect.Cue />
                                    </AppSelect.Activator>

                                    <AppSelect.Content>
                                        <template v-for="visual in visualStyles" :key="visual.id">
                                            <AppSelect.Item :value="visual">
                                                {{ `${visual.name}` }}
                                            </AppSelect.Item>
                                        </template>
                                    </AppSelect.Content>
                                </AppSelect.Root>
                            </SettingsItem>
                        </SettingsGroup>
                    </SettingsPage>
                </TransitionGroup>
            </div>
        </main>
    </div>
</template>