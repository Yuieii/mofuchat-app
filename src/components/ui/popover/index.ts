import { Popover } from "@vuetify/v0";
import AppPopoverContent from "./AppPopoverContent.vue";
import AppPopoverRoot from "./AppPopoverRoot.vue";

export const AppPopover = {
    Root: AppPopoverRoot,
    Activator: Popover.Activator,
    Content: AppPopoverContent,
};