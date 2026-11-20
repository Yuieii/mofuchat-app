import { toRef, watch } from "vue";
import { ConfigStore, MofuWindowStyles } from "../stores/config";

function getDarkMode() {
    console.log("Query: dark mode:", ConfigStore.data );
    return ConfigStore.data?.config.app.darkMode ?? false;
}

export function useTheme() {
    const isDark = toRef(() => { 
        return getDarkMode();
    });

    const isLight = toRef(() => { 
        return !getDarkMode();
    });

    const theme = toRef(() => {
        return isDark.value ? "dark" : "light";
    });

    return { isDark, isLight, theme };
}

export function useWindowStyling() {
    const currentStyling = toRef(() => { 
        console.log("Query: mofu:", ConfigStore.data );
        return ConfigStore.data?.config.mofu.windowStyle ?? "normal";
    });
    
    const isNormalStyling = toRef(() => {
        const style = currentStyling.value;
        return style == "normal";
    });

    const isMica = toRef(() => currentStyling.value == "mica");

    const isAcrylic = toRef(() => currentStyling.value == "acrylic");

    const lightBackground = toRef(() => {
        const table: Record<MofuWindowStyles, string> = {
            normal: "bg-[#fffe]",
            mica: "",
            acrylic: "bg-[#fff8]",
            vibrancy: "bg-[#fff5]",
            glass: "bg-[#fff5]"
        };

        return table[currentStyling.value];
    });

    const darkBackground = toRef(() => {
        const table: Record<MofuWindowStyles, string> = {
            normal: "bg-[#fffe]",
            mica: "",
            acrylic: "bg-[#fff8]",
            vibrancy: "",
            glass: ""
        };

        return table[currentStyling.value];
    });

    const defaultBackground = toRef(() => { 
        return getDarkMode() ? darkBackground.value : lightBackground.value;
    });

    watch(() => ConfigStore.data, (oldValue, newValue) => {
        console.log(oldValue, newValue);
    }, {
        deep: true
    });

    return { currentStyling, isNormalStyling, isMica, isAcrylic, defaultBackground };
}