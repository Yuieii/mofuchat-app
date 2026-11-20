import { invoke } from "@tauri-apps/api/core";
import { onMounted, onUnmounted, reactive } from "vue";
import { Log } from "../log";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export interface RGBColor {
    /** The red component ranged from 0 to 255. */
    r: number;
    /** The green component ranged from 0 to 255. */
    g: number;
    /** The blue component ranged from 0 to 255. */
    b: number;
}

export interface Service { 
    id: string;
    name: string;
    url: string;
    color: RGBColor;
    enabled: boolean;
}

export interface VersionMigrations { 
    version: string;
}

export interface InternalConfigModel { 
    migrations: VersionMigrations;
}

export interface SpeechConfig { 
    volume: number;
    rate: number;
    pitch: number;
}

export interface AppConfig { 
    darkMode: boolean;
}

export type MofuWindowStyles = "normal" | "mica" | "acrylic" | "vibrancy" | "glass";

export interface MofuConfig { 
    windowStyle: MofuWindowStyles;
}

export interface Config { 
    speech: SpeechConfig;
    app: AppConfig;
    mofu: MofuConfig;
}

export interface ConfigModel { 
    config: Config;
    __internal__: InternalConfigModel;
    services: Service[];
}

const logger = Log.createLogger("ConfigStore");

export const ConfigStore = reactive({
    data: <ConfigModel | null>null,
    isLoading: false,

    async loadConfig() {
        logger.log("Loading config...");
        this.isLoading = true;

        try {
            const config = await invoke<ConfigModel>("get_prefs");
            this.data = reactive(config);
            logger.log(config);
        } catch (ex) { 
            logger.error(ex);
        } finally {
            this.isLoading = false;
        }
    },

    async saveConfig() {
        logger.log("Saving config...");
        logger.log(this.data);
        await invoke("set_prefs", { value: this.data });
    },
});

export function useConfig() {
    const promise = listen("mofu://settings/invalidated", event => {
        const commandLabel = event.payload;
        const currentLabel = getCurrentWebviewWindow().label;
        
        if (commandLabel != currentLabel) {
            ConfigStore.loadConfig();
        }
    });

    onMounted(async () => { 
        await ConfigStore.loadConfig();
    });

    onUnmounted(async () => { 
        const unlisten = await promise;
        unlisten();
    });
}