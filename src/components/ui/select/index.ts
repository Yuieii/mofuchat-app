import { Select } from "@vuetify/v0";
import AppSelectActivator from "./AppSelectActivator.vue";
import AppSelectContent from "./AppSelectContent.vue";
import AppSelectItem from "./AppSelectItem.vue";
import AppSelectCue from "./AppSelectCue.vue";

export const AppSelect = {
    Root: Select.Root,
    Activator: AppSelectActivator,
    Content: AppSelectContent,
    Value: Select.Value,
    Cue: AppSelectCue,
    Item: AppSelectItem,
    Placeholder: Select.Placeholder,
};