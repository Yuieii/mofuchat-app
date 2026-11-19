import { listen } from "@tauri-apps/api/event";
import { reactive } from "vue";

import type { MofuEvent, MofuComment, MofuEventTypeLookup } from "./typings";
export * from "./typings";

export const EventStore = reactive({
    events: <MofuEvent[]>[]
});

listen<MofuEvent>("mofuchat-event", event => {
    EventStore.events.push(event.payload);
});