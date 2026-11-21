<script setup lang="ts">
import { Button, Switch } from "./ui/widgets";
import { FontAwesome } from "./ui/icons";
import { AlertDialogActionEvent, DragDropContext } from "@vuetify/v0";
import { watch, onBeforeUnmount, useTemplateRef, ref } from "vue";
import type { Service } from "../stores/config";
import { AppMenu } from "./ui/menu";
import { AppPopover } from "./ui/popover";
import ColorPicker from "./ui/widgets/ColorPicker.vue";
import { ConfigStore } from "../stores/config";
import { AppAlertDialog } from "./ui/dialog/index.ts";
import { invoke } from "@tauri-apps/api/core";
import { useDebouncedComputed } from "../composables/index.ts";

type Item = Service;
type Kinds = { type: "item", value: Item };

const { dnd , item } = defineProps<{
    dnd: DragDropContext<Kinds>,
    item: Item,
}>();

const el = useTemplateRef<HTMLElement>("el");

const ticket = dnd.draggables.register({
    el,
    type: "item",
    value: item
});

const deleteConfirmShown = ref(false);

function getClasses() {
    const defaults = "divide-x-1 border-[#0002] bg-white grid grid-cols-subgrid col-span-7 rounded-lg".split(" ");

    if (ticket.isDragging.value) {
        defaults.push("opacity-50");
    }

    return defaults.join(" ");
}

function getHex() {
    const color = serviceColor.value;
    const r = color.r;
    const g = color.g;
    const b = color.b;
    return "#" + [r, g, b].map(c => c.toString(16).padStart(2, "0")).join("");
}

function deleteService() {
    const config = ConfigStore.data;
    if (config == null) return;

    const index = config.services.findIndex(c => c.id === item.id);
    if (index == -1) return;

    config.services.splice(index, 1);
}

const serviceName = useDebouncedComputed({
    get() {
        return item.name;
    },
    set(value) {
        item.name = value;
    }
});

const serviceUrl = useDebouncedComputed({
    get() {
        return item.url;
    },
    set(value) {
        item.url = value;
    }
});

const urlSupported = ref(true);

watch([
    () => item.name,
    () => item.url,
    () => item.color,
], () => { 
    ConfigStore.saveConfig();
});

watch(() => item.url, async url => { 
    const supported = await invoke<boolean>("check_supports_url", { url });
    urlSupported.value = supported;
}, {
    immediate: true
});

watch(() => item.enabled, async () => { 
    await ConfigStore.saveConfig();
    await invoke("toggle_channel", {
        serviceId: item.id
    });
});

const serviceEnabled = useDebouncedComputed({
    get() {
        return item.enabled;
    },
    set(value) {
        item.enabled = value;
    }
}, 500);

const serviceColor = useDebouncedComputed({
    get() {
        return item.color;
    },
    set(value) {
        item.color = value;
    }
}, 500);

onBeforeUnmount(() => ticket.unregister());
</script>

<template>
    <div :class="getClasses()">
        <div class="border-[#0002] flex flex-row items-center px-3 pl-5 gap-3">
            <span ref="el" class="px-1"><FontAwesome.GripVertical class="fill-accent-300 h-4" /></span>
            <AppPopover.Root>
                <AppPopover.Activator class="inline-flex">
                    <span class="rounded-sm aspect-square w-5 my-2" :style="{ background: getHex() }"></span>
                </AppPopover.Activator>

                <AppPopover.Content class="rounded-3xl!">
                    <ColorPicker v-model="serviceColor" @change="(c) => serviceColor = c" />
                </AppPopover.Content>
            </AppPopover.Root>

            <!-- Service name. -->
            <input class="text-accent-700 grow" v-model="serviceName" />
        </div>
        <div class="border-[#0002] flex flex-row justify-stretch items-stretch">
            <!-- Service URL. -->
            <input class="disabled:opacity-50 disabled:cursor-not-allowed grow px-3 transition" 
                :class="urlSupported ? 'text-accent-700' : 'text-red-500 outline outline-red-500'"
                v-model="serviceUrl" 
                :disabled="item.enabled" />
        </div>
        <div class="border-[#0002] grid grid-cols-subgrid col-span-5">
            <div class="flex flex-row justify-center items-center px-3">
                <Switch />
            </div>
            <div class="flex flex-row justify-center items-center px-3">
                <Switch />
            </div>
            <div class="flex flex-row justify-center items-center px-3">
                <Switch />
            </div>
            <div class="flex flex-row justify-center items-center px-3">
                <Switch v-model="serviceEnabled" />
            </div>
            <div class="flex flex-row justify-center items-center gap-3 px-3">
                <Button disabled>
                  <FontAwesome.Comments class="h-4 fill-white" />
                </Button>
                
                <AppMenu.Root>
                    <AppMenu.Activator>
                        <Button>
                            <FontAwesome.Ellipsis class="h-4 fill-white" />
                        </Button>
                    </AppMenu.Activator>

                    <AppMenu.Content>
                        <AppMenu.Item>設定</AppMenu.Item>
                        <AppMenu.Item>在瀏覽器中開啟</AppMenu.Item>
                        <AppMenu.Separator />
                        <AppMenu.Item item-style="destructive" @click="deleteConfirmShown = true">移除</AppMenu.Item>
                    </AppMenu.Content>
                </AppMenu.Root>

                <AppAlertDialog.Root v-model="deleteConfirmShown">
                    <AppAlertDialog.Content>
                        <AppAlertDialog.SubContent>
                            <AppAlertDialog.Title>移除聊天室</AppAlertDialog.Title>
                            <AppAlertDialog.Description>確定要移除「{{ item.name }}」嗎？</AppAlertDialog.Description>
                        </AppAlertDialog.SubContent>
                        <AppAlertDialog.ActionBar>
                            <AppAlertDialog.Cancel>取消</AppAlertDialog.Cancel>
                            <AppAlertDialog.Action @action="deleteService()">移除</AppAlertDialog.Action>
                        </AppAlertDialog.ActionBar>
                    </AppAlertDialog.Content>
                </AppAlertDialog.Root>
            </div>
        </div>
    </div>

</template>