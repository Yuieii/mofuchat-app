import { createApp } from "vue";
import Settings from "./Settings.vue";

document.addEventListener("contextmenu", e => {
    e.preventDefault();
});

createApp(Settings).mount("#app");