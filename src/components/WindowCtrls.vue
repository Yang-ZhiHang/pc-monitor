<script lang="ts" setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const isMaximized = ref<Boolean>(false);
const toggleMaximize = async () => {
    isMaximized.value = !isMaximized.value;
    await invoke('window_toggle_maximize')
}

const minimize = () => invoke('window_minimize')

const isAlwaysOnTop = ref<Boolean>(false);
const toggleAlwaysOnTop = async () => {
    isAlwaysOnTop.value = !isAlwaysOnTop.value;
    await invoke('window_toggle_always_on_top', { is_always_on_top: isAlwaysOnTop.value })
}

const close = () => invoke('window_close');

const startDrag = (e: MouseEvent) => {
    // Prevent drag if the target is a button (minimize、maximize or close)
    if ((e.target as HTMLElement).tagName !== 'BUTTON') {
        invoke('window_start_drag');
    }
}
</script>

<template>
    <div class="flex items-center justify-end bg-dark-300 px-2 py-1 select-none gap-2" @mousedown="startDrag">
        <button @click="toggleAlwaysOnTop" :class="['ctrl-btn', isAlwaysOnTop ? 'text-blue-500' : '']">
            <i class="fa fa-thumbtack text-current pointer-events-none"></i>
        </button>
        <button @click="minimize" class="ctrl-btn">
            <i class="fa-solid fa-minus pointer-events-none"></i>
        </button>
        <button @click="toggleMaximize" class="ctrl-btn">
            <i v-if="!isMaximized" class="fa-regular fa-square pointer-events-none"></i>
            <i v-else class="fa-regular fa-clone pointer-events-none"></i>
        </button>
        <button @click="close" class="ctrl-btn close-btn">
            <i class="fa-solid fa-xmark pointer-events-none"></i>
        </button>
    </div>
</template>

<style scoped>
button {
    font-size: 1rem;
    width: 2rem;
    height: 2rem;
    border: none;
    border-radius: 3px;
    background: transparent;
    cursor: pointer;
    transition: all .1s ease-in-out;

    &:hover {
        background-color: #ffffff25;
    }
}
</style>