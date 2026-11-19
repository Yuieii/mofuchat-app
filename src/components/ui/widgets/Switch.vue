<script setup lang="ts">
import { Switch } from '@vuetify/v0';

const model = defineModel<boolean>({
    default: false
});
</script>

<template>
    <Switch.Root
      v-model="model"
      class="inline-flex items-center border-none bg-transparent p-0 outline-none rounded-full"
    >
      <Switch.Track
        class="relative inline-flex items-center w-11 h-6 rounded-full bg-surface-variant transition-colors bg-gray-300 data-[state=checked]:bg-accent-500"
      >
        <Switch.Thumb
          class="block size-4 rounded-full bg-white shadow-sm transition-transform translate-x-1 data-[state=checked]:translate-x-6"
        />
      </Switch.Track>
    </Switch.Root>
</template>