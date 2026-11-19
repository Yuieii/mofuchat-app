<script setup lang="ts">
const {
    min, max
} = defineProps<{
    min: number,
    max: number,
}>();

const emit = defineEmits<{
    (e: "commit"): void;
}>();

const model = defineModel({
    get(val: string | number) {
        if (typeof val === "string") return parseFloat(val);
        return val;
    },

    set(val: string | number) {
        if (typeof val === "string") return parseFloat(val);
        return val;
    }
});

function updateNumber(elem: HTMLInputElement) {
    if (elem.value.trim() == "") {
        model.value = 0;
    }

    const num = parseFloat(elem.value);
    if (isNaN(num)) return;

    if (typeof min !== "undefined" && num < min) {
        elem.value = `${min}`;
        model.value = min;
    }

    if (typeof max !== "undefined" && num > max) { 
        elem.value = `${max}`;
        model.value = max;
    }

    emit("commit");
}
</script>

<style lang="scss" scoped>
input {
    &::-webkit-outer-spin-button,
    &::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    -moz-appearance: textfield;
    appearance: textfield;
}
</style>

<template>
    <input class="w-fit text-right" v-bind="$attrs" :min :max type="number" v-model="model" @input="ev => updateNumber(ev.target as HTMLInputElement)" />
</template>
