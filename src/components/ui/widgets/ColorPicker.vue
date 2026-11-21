<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, Ref, ref, toRef, useTemplateRef, watch } from 'vue';
import Button from './Button.vue';
import ColorSlider from './ColorSlider.vue';
import { Utils } from '../../../utils.ts';
import { isNull } from '@vuetify/v0';
import NumberInput from './NumberInput.vue';
import { RGBColor } from '../../../stores/config.ts';

const extModel = defineModel<RGBColor>();

type ColorKind = "hsv" | "rgb";

const emit = defineEmits<{
    (e: "commit"): void;
    (e: "change", color: RGBColor): void;
}>();

interface SliderSpecification { 
    id: string;
    label: string;
    min: number;
    max: number;
    step?: number;
    valuePrefix?: string;
    valueSuffix?: string;
}

interface SliderSpecifications { 
    rgb: SliderSpecification[];
    hsv: SliderSpecification[];
}

const specs: SliderSpecifications = {
    rgb: [
        { id: "r", label: "R", min: 0, max: 255, step: 1 },
        { id: "g", label: "G", min: 0, max: 255, step: 1 },
        { id: "b", label: "B", min: 0, max: 255, step: 1 },
    ],
    hsv: [
        { id: "h", label: "色相", min: 0, max: 360, valueSuffix: "º" },
        { id: "s", label: "飽和度", min: 0, max: 100, valueSuffix: "%" },
        { id: "v", label: "明度", min: 0, max: 100, valueSuffix: "%" },
    ],
};

const rgb = reactive({ r: [extModel.value?.r ?? 0], g: [extModel.value?.g ?? 0], b: [extModel.value?.b ?? 0] });
const hsv = reactive((() => { 
    const { r, g, b } = extModel.value ?? { r: 0, g: 0, b: 0 };
    const [h, s, v] = Utils.rgbToHsv(r, g, b);
    return {
        h: [h * 360],
        s: [s * 100],
        v: [v * 100]
    };
})());

const models = {
    r: toRef(() => rgb.r),
    g: toRef(() => rgb.g),
    b: toRef(() => rgb.b),

    h: toRef(() => hsv.h),
    s: toRef(() => hsv.s),
    v: toRef(() => hsv.v),
};

const currentColorMode = ref<ColorKind>("rgb");

const preview = useTemplateRef<HTMLElement>("preview");
const picker = useTemplateRef<HTMLElement>("picker");

function getHex() {
    const r = models.r.value;
    const g = models.g.value;
    const b = models.b.value;
    return "#" + [r, g, b].map(c => c[0].toString(16).padStart(2, "0")).join("");
}

function setCurrentColorMode(mode: ColorKind) { 
    currentColorMode.value = mode;

    switch (mode) {
        case "rgb":
            rgb.r = models.r.value;
            rgb.g = models.g.value;
            rgb.b = models.b.value;

            models.r = toRef(() => rgb.r);
            models.g = toRef(() => rgb.g);
            models.b = toRef(() => rgb.b);

            models.h = computed(() => { 
                return [Math.round(Utils.rgbToHsv(rgb.r[0], rgb.g[0], rgb.b[0])[0] * 360)];
            });
            models.s = computed(() => {
                return [Math.round(Utils.rgbToHsv(rgb.r[0], rgb.g[0], rgb.b[0])[1] * 100)];
            });
            models.v = computed(() => {
                return [Math.round(Utils.rgbToHsv(rgb.r[0], rgb.g[0], rgb.b[0])[2] * 100)];
            });
            break;

        case "hsv":
            hsv.h = models.h.value;
            hsv.s = models.s.value;
            hsv.v = models.v.value;

            models.h = toRef(() => hsv.h);
            models.s = toRef(() => hsv.s);
            models.v = toRef(() => hsv.v);

            models.r = computed(() => { 
                return [Math.round(Utils.hsvToRgb(hsv.h[0] / 360, hsv.s[0] / 100, hsv.v[0] / 100)[0])];
            });
            models.g = computed(() => { 
                return [Math.round(Utils.hsvToRgb(hsv.h[0] / 360, hsv.s[0] / 100, hsv.v[0] / 100)[1])];
            });
            models.b = computed(() => { 
                return [Math.round(Utils.hsvToRgb(hsv.h[0] / 360, hsv.s[0] / 100, hsv.v[0] / 100)[2])];
            });
            break;
    }
}

setCurrentColorMode("hsv");

function model(slider: SliderSpecification): Ref<[number]> {
    // @ts-ignore
    return models[slider.id];
}

watch(rgb, n => { 
    emit("change", collectColor());
});

watch(hsv, n => { 
    emit("change", collectColor());
});

function collectColor() {
    return {
        r: models.r.value[0],
        g: models.g.value[0],
        b: models.b.value[0],
        h: models.h.value[0],
        s: models.s.value[0],
        l: models.v.value[0],
    };
}

function getGradient(slider: SliderSpecification): string { 
    switch (slider.id) {
        case "h":
            const stops = <string[]>[];
            const stopsCount = 16;
            for (let i = 0; i <= stopsCount; i++) { 
                const deg = i / stopsCount * 360;
                stops.push(`hsl(${deg}deg 100% 50%)`);
            }

            const joined = stops.join(", ");
            return `linear-gradient(to right, ${joined})`;
        case "s": {
            const fromColor = Utils.hsvToRgb(models.h.value[0] / 360, 0, models.v.value[0] / 100);
            const toColor = Utils.hsvToRgb(models.h.value[0] / 360, 1, models.v.value[0] / 100);
            return `linear-gradient(to right, rgb(${fromColor[0]} ${fromColor[1]} ${fromColor[2]}), rgb(${toColor[0]} ${toColor[1]} ${toColor[2]}))`;
        }
        case "v": {
            const fromColor = Utils.hsvToRgb(models.h.value[0] / 360, models.s.value[0] / 100, 0);
            const toColor = Utils.hsvToRgb(models.h.value[0] / 360, models.s.value[0] / 100, 1);
            return `linear-gradient(to right, rgb(${fromColor[0]} ${fromColor[1]} ${fromColor[2]}%), rgb(${toColor[0]} ${toColor[1]} ${toColor[2]}))`;
        }
        case "r":
            return `linear-gradient(to right, rgb(0 ${models.g.value[0]} ${models.b.value[0]}), rgb(255 ${models.g.value[0]} ${models.b.value[0]}))`;
        case "g":
            return `linear-gradient(to right, rgb(${models.r.value[0]} 0 ${models.b.value[0]}), rgb(${models.r.value[0]} 255 ${models.b.value[0]}))`;
        case "b":
            return `linear-gradient(to right, rgb(${models.r.value[0]} ${models.g.value[0]} 0), rgb(${models.r.value[0]} ${models.g.value[0]} 255))`;
    }

    return "#000";
}

function getThumbColor(slider: SliderSpecification): string { 
    switch (slider.id) {
        case "h":
            return `hsl(${models.h.value[0]}deg 100% 50%)`;
        case "s":
        case "v":
            // return `hsl(${models.h.value[0]}deg ${models.s.value[0]}%  ${models.v.value[0]}%)`;

        case "r":
        case "g":
        case "b":
            return `rgb(${models.r.value[0]} ${models.g.value[0]} ${models.b.value[0]})`;
    }

    return "#000";
}

function getPreviewStyle() {
    return {
        "--mf-hue": models.h.value[0] + 'deg'
    }
}

function getPickerStyle() {
    const outer = preview.value;
    const point = picker.value;
    if (isNull(outer) || isNull(point)) return undefined;

    // const rect = outer.getBoundingClientRect();
    
    return {
        left: `${models.s.value[0] / 100 * 280}px`,
        top: `${(1 - models.v.value[0] / 100) * 180}px`,
    }
}

function handleMouseMove(event: MouseEvent) { 
    const outer = preview.value;
    const point = picker.value;
    if (isNull(outer) || isNull(point)) return undefined;

    if ((event.buttons & 1) === 1) {
        const rect = outer.getBoundingClientRect();

        function clamp01(n: number) {
            return n > 1 ? 1 : n < 0 ? 0 : n; 
        }

        const h = models.h.value[0];
        const s = clamp01((event.clientX - rect.x) / rect.width);
        const l = clamp01(1 - ((event.clientY - rect.y) / rect.height));

        const [r, g, b] = Utils.hsvToRgb(h, s, l);

        rgb.r[0] = Math.round(r);
        rgb.g[0] = Math.round(g);
        rgb.b[0] = Math.round(b);

        hsv.h[0] = Math.round(h);
        hsv.s[0] = Math.round(s * 100);
        hsv.v[0] = Math.round(l * 100);
    }
}

function handleMouseUp(event: MouseEvent) {
    document.body.removeEventListener("mousemove", handleMouseMove);
    emit("commit");
}

function handleMouseOut(event: MouseEvent) {
    document.body.removeEventListener("mousemove", handleMouseMove);
}

function handleMouseDown(event: MouseEvent) {
    document.body.addEventListener("mousemove", handleMouseMove);
    document.body.addEventListener("mouseup", handleMouseUp);
    document.addEventListener("mouseleave", handleMouseOut);
}

onMounted(() => { 
    const outer = preview.value;
    const point = picker.value;
    if (isNull(outer) || isNull(point)) return;

    outer.addEventListener("mousedown", handleMouseDown);
});

onUnmounted(() => {
    const outer = preview.value;
    const point = picker.value;
    if (isNull(outer) || isNull(point)) return;

    outer.removeEventListener("mousedown", handleMouseDown);
    document.body.removeEventListener("mousemove", handleMouseMove);
    document.body.removeEventListener("mouseup", handleMouseUp);
    document.removeEventListener("mouseleave", handleMouseOut);
});
</script>

<style lang="scss" scoped>
.mf-color-picker-slice {
    --mf-hue: 0deg;

    background: 
        linear-gradient(to bottom, transparent, black),
        linear-gradient(to right in hsl, hsl(var(--mf-hue) 0% 100%), hsl(var(--mf-hue) 100% 50%));
}
</style>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row">
            <div ref="preview" class="mf-color-picker-slice relative w-[280px] h-[180px]" :style="getPreviewStyle()">
                <div ref="picker" class="absolute -translate-x-1/2 -translate-y-1/2 w-5 h-5 border-2 border-black outline-2 outline-white drop-shadow-[0_0_0.1em] drop-shadow-white rounded-full" :style="getPickerStyle()"></div>
            </div>

            <div class="flex flex-col text-sm px-3 py-2 gap-2">
                <div class="flex flex-col">
                    <span class="text-accent-500 opacity-70">HSL</span>
                    <span class="select-text text-accent-500 w-36">{{ models.h.value[0] }}º, {{ models.s.value[0] }}%, {{ models.v.value[0] }}%</span>
                </div>
                <div class="flex flex-col">
                    <span class="text-accent-500 opacity-70">RGB</span>
                    <span class="select-text text-accent-500">{{ getHex() }}</span>
                </div>
            </div>
        </div>

        <div class="flex flex-col items-stretch bg-accent-100">
            <div class="flex flex-row justify-center gap-2 p-2">
                <Button @click="setCurrentColorMode('hsv')" :disabled="currentColorMode == 'hsv'" class="text-xs">HSL</Button>
                <Button @click="setCurrentColorMode('rgb')" :disabled="currentColorMode == 'rgb'" class="text-xs">RGB</Button>
            </div>

            <div class="flex flex-col gap-1 bg-white m-3 mt-0 px-0 pt-2 pb-2 rounded-xl">
                <div class="flex flex-col" v-for="slider in specs[currentColorMode]" :key="slider.id">
                    <div class="flex flex-row mx-3 text-accent-500 text-sm justify-between">
                        <span>{{ slider.label }}</span>
                        <span>
                            {{ slider.valuePrefix ?? "" }}
                            <NumberInput :min="slider.min" :max="slider.max"  v-model="model(slider).value[0]" />
                            {{ slider.valueSuffix ?? "" }}
                        </span>
                    </div>
                    <ColorSlider 
                        :min="slider.min" :max="slider.max" v-model="model(slider).value[0]" 
                        :gradient="getGradient(slider)" :thumb-color="getThumbColor(slider)"
                        @commit="emit('commit')" />
                </div>
            </div>
        </div>
    </div>
</template>