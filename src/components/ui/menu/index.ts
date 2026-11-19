import { Menu } from "@vuetify/v0";
import AppMenuContent from "./AppMenuContent.vue";
import AppMenuItem from "./AppMenuItem.vue";
import AppMenuSubActivator from "./AppMenuSubActivator.vue";
import AppMenuSubContent from "./AppMenuSubContent.vue";
import AppMenuSeparator from "./AppMenuSeparator.vue";
import AppMenuRoot from "./AppMenuRoot.vue";

export const AppMenu = {
    Root: AppMenuRoot,
    Sub: Menu.Sub,
    Activator: Menu.Activator,
    SubActivator: AppMenuSubActivator,
    Item: AppMenuItem,
    Content: AppMenuContent,
    SubContent: AppMenuSubContent,
    Separator: AppMenuSeparator,
};