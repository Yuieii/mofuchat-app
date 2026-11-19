<script setup lang="ts">
import "./assets/scss/style.scss";
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { AlertDialog, Menu, Splitter } from "@vuetify/v0";
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

import { FontAwesome } from "./components/ui/icons/index.ts";

import ServiceItem from "./components/ServiceItem.vue";
import ContainerDebug from "./components/ContainerDebug.vue";
import ServiceList from "./components/ServiceList.vue";
import { Button }  from "./components/ui/widgets";
import StreamList from "./components/StreamList.vue";
import { AppMenu } from "./components/ui/menu";
import { AppSplitter } from "./components/ui/splitter/index.ts";

import { ConfigStore } from "./stores/config.ts";
import { Log } from "./log.ts";
import { listen } from "@tauri-apps/api/event";
import EventsView from "./components/EventsView.vue";

const logger = Log.createLogger("App");

interface MofuEvents { 

}

/*
        "trafficLightPosition": {
          "x": 20,
          "y": 22
        } 
 */

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
    color: { r: 0, g: 0, b: 0 }
  });

  logger.log("Created new service");
}

onMounted(async () => {
  await ConfigStore.loadConfig();
});
</script>

<template>
  <div class="flex flex-col min-h-0 min-w-0 w-screen h-screen max-w-screen max-h-screen overscroll-none overflow-hidden bg-[#fff8]">
    <header data-tauri-drag-region class="shrink-0 relative flex flex-col items-stretch bg-accent-600 text-white font-black text-[95%]">
      <!-- Title bar-->

      <div data-tauri-drag-region class="flex flex-row h-full w-[8em] left-[calc(50vw_-_4em)] items-center justify-center absolute"> 
        <!-- Title bar center, floating on top -->
        <div data-tauri-drag-region class="flex flex-row w-screen items-center justify-center grow">
          <span data-tauri-drag-region class="opacity-50 text-[90%]">MofuChat</span>
        </div>
      </div>

      <div data-tauri-drag-region class="flex flex-row items-center justify-between">
        <div data-tauri-drag-region class="flex flex-row px-3 py-3 items-stretch gap-2">
          <!-- Title bar left side -->
          <div data-tauri-drag-region class="opacity-0 flex flex-row items-stretch px-2 bg-green-500">
            <div class="w-[55px] h-full bg-red-500"></div>
          </div>

          <div class="flex flex-row gap-2">
            <Button @click="createNewService()">新增</Button>
          </div>
        </div>

        <div data-tauri-drag-region class="flex flex-row px-3 py-3">
          <!-- Title bar right side -->
          <div class="flex flex-row gap-2">
            <Button>觀眾名單</Button>
            <Button>停止朗讀</Button>
            <Button>清除留言</Button>

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
          </div>
        </div>
      </div>
    </header>

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