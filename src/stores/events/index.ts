import { listen } from "@tauri-apps/api/event";
import { onUnmounted, reactive } from "vue";

import { MofuAppEvent, MofuAppEventTypeLookup, MofuComment, type MofuEvent } from "./typings";
import { Utils } from "../../utils";
import { invoke } from "@tauri-apps/api/core";
export * from "./typings";

export const EventStore = reactive({
    events: <(MofuEvent & { key: string })[]>[]
});

export class EventBus<I extends object> extends EventTarget {
    public addEventListener<K extends keyof I>(type: K, callback: (ev: I[K]) => void): void;
    public addEventListener(type: string, callback: (ev: unknown) => void, options?: AddEventListenerOptions | boolean): void
    public addEventListener(type: string, callback: (ev: unknown) => void, options?: AddEventListenerOptions | boolean): void {
        super.addEventListener(`mofu://${type}`, ev => {
            // `ev` should be a CustomEvent since we always use that.
            if (ev instanceof CustomEvent) {
                callback(ev.detail);
            }
        }, options);
    }

    public on<K extends keyof I>(type: K, listener: (ev: I[K]) => void): void;
    public on(type: string, listener: (ev: unknown) => void, options?: boolean | AddEventListenerOptions): void;
    public on(type: string, listener: (ev: unknown) => void, options?: boolean | AddEventListenerOptions): void {
        super.addEventListener(`mofu://${type}`, ev => {
            // `ev` should be a CustomEvent since we always use that.
            if (ev instanceof CustomEvent) {
                listener(ev.detail);
            }
        }, options);
    }

    public dispatch<K extends keyof I>(type: K, payload: I[K]): void;
    public dispatch(type: string, payload: unknown): void;
    public dispatch(type: string, payload: unknown): void {
        super.dispatchEvent(new CustomEvent(`mofu://${type}`, {
            detail: payload
        }));
    }
}

export function useMofuAppEvents() {
    const eventBus = new EventBus<MofuAppEventTypeLookup>();
    const promise = listen<MofuAppEvent>("mofu://app/event", event => {
        eventBus.dispatch(event.payload.event, event.payload.data);
    });

    onUnmounted(async () => {
        const unlisten = await promise;
        unlisten();
    });

    return { eventBus };
}

export function useMofuServiceEvents() {
    const promise = listen<MofuEvent>("mofu://services/event", event => {
        EventStore.events.push({
            ...event.payload,
            key: Utils.createUuid()
        });

        const threshold = 1000;
        if (EventStore.events.length > threshold) {
            EventStore.events.splice(0, EventStore.events.length - threshold);
        }
    });

    onUnmounted(async () => {
        const unlisten = await promise;
        unlisten();
    });
}

