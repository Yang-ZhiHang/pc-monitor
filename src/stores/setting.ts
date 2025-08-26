import { defineStore } from "pinia";
import { ref } from "vue";
import { useI18n } from 'vue-i18n';

export const useSettingStore = defineStore('setting', () => {
    const { locale } = useI18n();
    const lang = ref<string>('en');
    const startOnBoot = ref<boolean>(false);

    function switchLang(l: string) {
        lang.value = l;
        locale.value = l;
    }
    return { lang, startOnBoot, switchLang };
}, { persist: true });