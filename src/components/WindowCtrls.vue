<script lang="ts" setup>
import { ref, toRefs } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { navItems, githubLink } from '@/constants/nav';
import { open } from '@tauri-apps/plugin-shell';

import { useI18n } from 'vue-i18n';
import { useSettingStore } from '@/stores/setting';
const settingStore = useSettingStore();
const { t } = useI18n();

const isMaximized = ref<Boolean>(false);
const toggleMaximize = async () => {
    isMaximized.value = !isMaximized.value;
    await invoke<void>('window_toggle_maximize')
}

const minimize = () => invoke<void>('window_minimize')

const isAlwaysOnTop = ref<Boolean>(false);
const toggleAlwaysOnTop = async () => {
    isAlwaysOnTop.value = !isAlwaysOnTop.value;
    await invoke<void>('window_toggle_always_on_top', { is_always_on_top: isAlwaysOnTop.value })
}

const close = () => invoke<void>('window_close', { hide: settingStore.minToTray });

const startDrag = (e: MouseEvent) => {
    // Prevent drag if the target is a button (minimize、maximize or close)
    if ((e.target as HTMLElement).tagName !== 'BUTTON') {
        invoke<void>('window_start_drag');
    }
}

const props = defineProps({
    currentRoute: {
        type: String,
        required: true
    }
});

const { currentRoute } = toRefs(props);

const getTitle = (): string => {
    for (const item of navItems) {
        if (item.route === currentRoute.value) {
            return item.detail;
        }
    }
    return '';
}
</script>

<template>
    <div class="flex flex-col items-center justify-between bg-dark-400 px-2 py-1 select-none gap-2 border-b border-dark-200"
        @mousedown="startDrag">
        <div class="flex justify-end w-full">
            <button @click="toggleAlwaysOnTop" :class="isAlwaysOnTop ? 'text-blue-500' : ''">
                <i class="fa fa-thumbtack text-current"></i>
            </button>
            <button @click="minimize">
                <i class="fa-solid fa-minus"></i>
            </button>
            <button @click="toggleMaximize">
                <i v-if="!isMaximized" class="fa-regular fa-square"></i>
                <i v-else class="fa-regular fa-clone"></i>
            </button>
            <button @click="close" class="btn-close">
                <i class=" fa-solid fa-xmark"></i>
            </button>
        </div>
        <div class="flex items-center justify-between w-full pl-4 pr-2">
            <h1 class="text-xl font-bold w-max shrink-0">{{ t(getTitle()) }}</h1>
            <div class="flex justify-end w-full">
                <button @click="() => open(githubLink)" class="rounded-full! p-1" :title="t('nav.github')">
                    <i class="fa-brands fa-github text-2xl aspect-square flex! items-center justify-center"></i>
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
button {
    font-size: 1rem;
    min-width: 2rem;
    aspect-ratio: 1/1;
    border: none;
    border-radius: 3px;
    background: transparent;
    cursor: pointer;
    transition: all .1s ease-in-out;

    &:hover {
        background-color: #ffffff25;
    }

    i {
        pointer-events: none;
    }
}

.btn-close {
    &:hover {
        background-color: rgba(255, 1, 1, 0.386);
    }
}
</style>