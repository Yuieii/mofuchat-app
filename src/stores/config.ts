import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";
import { Log } from "../log";

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

export interface ConfigModel { 
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
        await invoke("set_prefs", { value: this.data });
    },
});