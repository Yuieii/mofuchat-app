import { AlertDialog } from "@vuetify/v0";
import AppAlertDialogRoot from "./AppAlertDialogRoot.vue";
import AppAlertDialogContent from "./AppAlertDialogContent.vue";
import AppAlertDialogTitle from "./AppAlertDialogTitle.vue";
import AppAlertDialogDescription from "./AppAlertDialogDescription.vue";
import AppAlertDialogActionBar from "./AppAlertDialogActionBar.vue";
import AppAlertDialogAction from "./AppAlertDialogAction.vue";
import AppAlertDialogCancel from "./AppAlertDialogCancel.vue";
import AppAlertDialogClose from "./AppAlertDialogClose.vue";
import AppAlertDialogSubContent from "./AppAlertDialogSubContent.vue";

export const AppAlertDialog = {
    Root: AppAlertDialogRoot,
    Activator: AlertDialog.Activator,
    Title: AppAlertDialogTitle,
    Description: AppAlertDialogDescription,
    ActionBar: AppAlertDialogActionBar,
    Action: AppAlertDialogAction,
    Cancel: AppAlertDialogCancel,
    Close: AppAlertDialogClose,
    Content: AppAlertDialogContent,
    SubContent: AppAlertDialogSubContent,
};