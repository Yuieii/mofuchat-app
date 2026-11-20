import { listen } from "@tauri-apps/api/event";
import { onUnmounted, reactive } from "vue";

import type { MofuEvent } from "./typings";
import { Utils } from "../../utils";
export * from "./typings";

export const EventStore = reactive({
    events: <(MofuEvent & { key: string })[]>[]
});

export function useMofuEvents() {
    const promise = listen<MofuEvent>("mofu://services/event", event => {
        EventStore.events.push({
            ...event.payload,
            key: Utils.createUuid()
        });
    });

    onUnmounted(async () => {
        const unlisten = await promise;
        unlisten();
    });
}

