import { defineStore } from "pinia";
import { ref, watch } from "vue";
import { useI18n } from 'vue-i18n';
import { invoke } from "@tauri-apps/api/core";

export const useSettingStore = defineStore('setting', () => {
    const { locale } = useI18n();
    const lang = ref<string>('en');
    const startOnBoot = ref<boolean>(false);

    function switchLang(l: string) {
        lang.value = l;
        locale.value = l;
    }

    watch(startOnBoot, (newVal) => {
        invoke('set_start_on_boot_rs', { enable: newVal });
    });

    return { lang, startOnBoot, switchLang };
}, { persist: true });