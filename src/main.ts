import { createApp } from "vue";
import App from "./App.vue";
import router from './router/index';

import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

import ElementPlus from 'element-plus'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './style/base.css'

import { createI18n } from 'vue-i18n'
import zh from '@/locales/zh.json'
import en from '@/locales/en.json'
const setting = localStorage.getItem('setting');
const savedLang = setting ? JSON.parse(setting).lang : 'en';
const i18n = createI18n({
    legacy: false,
    locale: savedLang,
    messages: {
        zh, en
    }
})

const app = createApp(App);
app.use(router);
app.use(pinia)
app.use(ElementPlus);
app.use(i18n);
app.mount("#app");
