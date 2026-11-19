import type { ComputedRef, InjectionKey, Ref } from "vue";

const splitterOrientation: InjectionKey<Ref<"vertical" | "horizontal">> = Symbol("mofu.splitter.orientation");

export const Contexts = {
    splitterOrientation
};