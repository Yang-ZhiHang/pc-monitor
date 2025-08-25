import { createApp } from "vue";
import App from "./App.vue";
import router from './router/index';
import './style/base.css'

import { createI18n } from 'vue-i18n'
import zh from '@/locales/zh.json'
import en from '@/locales/en.json'
const i18n = createI18n({
    legacy: false,
    locale: 'en',
    messages: {
        zh, en
    }
})

const app = createApp(App);
app.use(router);
app.use(i18n);
app.mount("#app");
