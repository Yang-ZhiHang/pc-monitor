import { createApp } from "vue";
import App from "./App.vue";
import router from './router/index';
import ElementPlus from 'element-plus'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './style/base.css'

import { createI18n } from 'vue-i18n'
import zh from '@/locales/zh.json'
import en from '@/locales/en.json'
const savedLang = localStorage.getItem('app_language') || 'en';
const i18n = createI18n({
    legacy: false,
    locale: savedLang,
    messages: {
        zh, en
    }
})

const app = createApp(App);
app.use(router);
app.use(i18n);
app.use(ElementPlus);
app.mount("#app");
