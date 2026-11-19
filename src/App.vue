<script setup lang="ts">
import "./assets/scss/style.scss";
import { onMounted } from "vue";
import ContainerDebug from "./components/ContainerDebug.vue";
import ServiceList from "./components/ServiceList.vue";
import StreamList from "./components/StreamList.vue";
import { AppSplitter } from "./components/ui/splitter/index.ts";

import { ConfigStore } from "./stores/config.ts";
import EventsView from "./components/EventsView.vue";
import TitleBar from "./components/TitleBar.vue";
import { Log } from "./log.ts";
import { platform } from "@tauri-apps/plugin-os";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Button from "./components/ui/widgets/Button.vue";
import { AppMenu } from "./components/ui/menu/index.ts";
import { FontAwesome } from "./components/ui/icons/index.ts";
import { AppTooltip } from "./components/ui/tooltip/index.ts";


const logger = Log.createLogger("App");

const currentPlatform = platform();

function openSettings() {
    const webview = new WebviewWindow("settings", {
        url: "/settings.html",
        title: "Settings",
        fullscreen: false,
        resizable: true,
        width: 800,
        minWidth: 800,
        height: 600,
        minHeight: 600,
        titleBarStyle: "overlay",
        hiddenTitle: true,
        center: true,
        focusable: true,
        focus: true,
        decorations: currentPlatform === "macos"
    });
}

function createUuid() {
    const chars = "0123456789abcdef".split("");
    const result = <string[]>[];
    for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 2; j++) {
            result.push(chars[Math.floor(Math.random() * chars.length)]);
        }

        if ([4, 6, 8, 10].includes(i + 1)) {
            result.push("-");
        }
    }

    return result.join("");
}

function createNewService() {
    const services = ConfigStore.data?.services;
    if (services == null) return;
    services.push({
        id: createUuid(),
        name: "#",
        url: "https://",
        color: { r: 0, g: 0, b: 0 },
        enabled: true,
    });

    logger.log("Created new service");
}

onMounted(async () => {
    await ConfigStore.loadConfig();
});
</script>

<template>
    <div class="flex flex-col min-h-0 min-w-0 w-screen h-screen max-w-screen max-h-screen overscroll-none overflow-hidden bg-[#fff8]">
        <TitleBar>
            <template #left>
                <Button @click="createNewService()">新增</Button>
            </template>

            <template #right>
                <!--
                    <Button>觀眾名單</Button>
                    <Button>停止朗讀</Button>
                    <Button>清除留言</Button>
                -->

                <div class="divide-x-1 shrink-0 bg-accent-500 text-white fill-white leading-1 rounded-full text-sm outline-none overflow-hidden">
                    <AppTooltip.Root class="border-accent-400" position-area="bottom">
                        <AppTooltip.Activator>
                            <button class="px-[0.9em] py-[0.4em] disabled:opacity-50 enabled:hover:bg-accent-400 enabled:active:bg-accent-600 transition duration-200">
                                <FontAwesome.Trash class="h-4" />
                            </button>
                        </AppTooltip.Activator>
                        <AppTooltip.Content>
                            <span>觀眾名單</span>
                        </AppTooltip.Content>
                    </AppTooltip.Root>
                    <AppTooltip.Root class="border-accent-400" position-area="bottom">
                        <AppTooltip.Activator>
                            <button class="px-[0.9em] py-[0.4em] disabled:opacity-50 enabled:hover:bg-accent-400 enabled:active:bg-accent-600 transition duration-200">
                                <FontAwesome.CircleStop class="h-4" />
                            </button>
                        </AppTooltip.Activator>
                        <AppTooltip.Content>
                            <span>停止朗讀</span>
                        </AppTooltip.Content>
                    </AppTooltip.Root>
                    <AppTooltip.Root class="border-accent-400" position-area="bottom">
                        <AppTooltip.Activator>
                            <button class="px-[0.9em] py-[0.4em] disabled:opacity-50 enabled:hover:bg-accent-400 enabled:active:bg-accent-600 transition duration-200">
                                <FontAwesome.Trash class="h-4" />
                            </button>
                        </AppTooltip.Activator>
                        <AppTooltip.Content>
                            <span>清除留言</span>
                        </AppTooltip.Content>
                    </AppTooltip.Root>
                </div>

                <AppMenu.Root>
                    <AppMenu.Activator>
                        <Button>
                            <FontAwesome.Ellipsis class="h-4 fill-white" />
                        </Button>
                    </AppMenu.Activator>

                    <AppMenu.Content position-area="bottom left">
                        <AppMenu.Item @click="openSettings()">設定</AppMenu.Item>
                        <AppMenu.Item>模板</AppMenu.Item>
                        <AppMenu.Item>留言測試器</AppMenu.Item>
                        <AppMenu.Item>字典</AppMenu.Item>
                        <AppMenu.Item>整合</AppMenu.Item>
                        <AppMenu.Item>互動管理</AppMenu.Item>
                        <AppMenu.Item>歌單曲目</AppMenu.Item>
                        <AppMenu.Item>問卷調查</AppMenu.Item>
                        <AppMenu.Item>備份</AppMenu.Item>
                        <AppMenu.Item>遠端觀看</AppMenu.Item>
                        <AppMenu.Item>時間戳</AppMenu.Item>
                        <AppMenu.Item>留言紀錄</AppMenu.Item>
                        <AppMenu.Item>擴充功能</AppMenu.Item>
                        <AppMenu.Separator />
                        <AppMenu.Item>開啟資料夾</AppMenu.Item>
                        <AppMenu.Item>通知</AppMenu.Item>
                        <AppMenu.Item>支援</AppMenu.Item>

                        <AppMenu.Sub class="drop-shadow-black">
                            <AppMenu.SubActivator>其他</AppMenu.SubActivator>

                            <AppMenu.SubContent>
                                <AppMenu.Item>關於</AppMenu.Item>
                            </AppMenu.SubContent>
                        </AppMenu.Sub>
                    </AppMenu.Content>
                </AppMenu.Root>
            </template>
        </TitleBar>

        <main class="flex flex-col grow min-h-0 min-w-0 max-w-screen">
            <div class="flex flex-row grow min-w-0 min-h-0">
                <div class="flex flex-col grow min-w-0 min-h-0">

                    <AppSplitter.Root orientation="vertical">
                        <AppSplitter.Panel :default-size="33" :min-size="10">
                            <ServiceList />
                        </AppSplitter.Panel>

                        <AppSplitter.Handle />

                        <AppSplitter.Panel :default-size="34" :min-size="10">

                                <AppSplitter.Root class="h-full" orientation="horizontal">
                                    <AppSplitter.Panel :default-size="50" :min-size="30">
                                        <div class="flex flex-col max-h-full">
                                            <div class="flex flex-row justify-between items-center shrink-0 bg-white backdrop-blur-xl z-1 border-b-1 border-b-[#0003]">
                                                <span class="text-accent-400 font-black text-sm px-3 py-2">事件</span>
                                                <div class="text-accent-400 font-black text-sm px-3 py-2">Tools</div>
                                            </div>
                                            <div class="grow overflow-x-hidden overflow-y-auto overscroll-contain min-h-0">
                                                <div class="flex flex-col min-h-0">
                                                    <EventsView />
                                                </div>
                                            </div>
                                        </div>
                                    </AppSplitter.Panel>
                                    
                                    <AppSplitter.Handle />

                                    <AppSplitter.Panel :default-size="50" :min-size="30">
                                        <div class="flex flex-col w-full h-full max-w-full max-h-full">
                                            <div class="flex flex-row justify-between items-center shrink-0 bg-white backdrop-blur-xl z-1 border-b-1 border-b-[#0003]">
                                                <span class="text-accent-400 font-black text-sm px-3 py-2">禮物/超級留言</span>
                                                <div class="text-accent-400 font-black text-sm px-3 py-2">Tools</div>
                                            </div>

                                            <div class="flex flex-row min-w-0 min-h-0 max-h-full grow">
                                                <div class="flex flex-col min-w-0 min-h-0 max-w-full grow">
                                                    <AppSplitter.Root class="h-full mr-2" orientation="horizontal">
                                                        <AppSplitter.Panel :default-size="30" :min-size="20">
                                                            <div class="flex flex-row min-w-0 max-h-full">
                                                                <div class="flex grow overflow-x-hidden overflow-y-auto overscroll-contain min-w-0 min-h-0">
                                                                    <ContainerDebug />
                                                                </div>
                                                            </div>
                                                        </AppSplitter.Panel>
                                                        <AppSplitter.Handle />
                                                        <AppSplitter.Panel :default-size="30" :min-size="20">
                                                            <div class="flex flex-row min-w-0 max-h-full">
                                                                <div class="flex grow overflow-x-hidden overflow-y-auto overscroll-contain min-w-0 min-h-0">
                                                                    <ContainerDebug />
                                                                </div>
                                                            </div>
                                                        </AppSplitter.Panel>
                                                    </AppSplitter.Root>
                                                </div>
                                            </div>
                                        </div>
                                    </AppSplitter.Panel>
                                </AppSplitter.Root>

                        </AppSplitter.Panel>
                        
                        <AppSplitter.Handle />

                        <AppSplitter.Panel :default-size="33" :min-size="10">
                            <div class="flex flex-col w-full h-full max-w-full max-h-full">
                                <div class="hidden flex flex-row justify-between items-center shrink-0 bg-white backdrop-blur-xl z-1 border-b-1 border-b-[#0003]">
                                    <span class="text-accent-400 font-black text-sm px-3 py-2">禮物/超級留言</span>
                                    <div class="text-accent-400 font-black text-sm px-3 py-2">Tools</div>
                                </div>
                                <div class="grow flex flex-row overflow-y-hidden min-h-0">
                                    <div class="flex flex-row max-h-full min-h-0">
                                        <StreamList />
                                    </div>
                                </div>
                            </div>
                        </AppSplitter.Panel>
                    </AppSplitter.Root>
                </div>
            </div>
         
        </main>
    </div>
</template>