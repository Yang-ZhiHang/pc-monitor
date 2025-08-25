<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
const { t, locale } = useI18n();
import { ref } from 'vue';

const selectedLanguage = ref('zh');

const settings = ref({
  startOnBoot: false
});

const switchLang = (lang: string) => {
  locale.value = lang;
}

const saveSettings = () => {
  switchLang(selectedLanguage.value);
};
</script>

<template>
  <div class="min-h-min p-6 animate-fade">
    <div class="bg-dark-200 rounded-xl p-6 card-shadow h-full max-w-2xl mx-auto">
      <h2 class="text-xl font-semibold mb-6 text-light-300">{{ t('setting.title') }}</h2>

      <div class="space-y-6 text-light-300">
        <div>
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-globe text-primary mr-2"></i>
            {{ t('setting.language.title') }}
          </h3>
          <div class="pl-6">
            <select v-model="selectedLanguage"
              class="w-full bg-dark-300 border border-dark-100 rounded-md mr-3 pl-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary cursor-pointer">
              <option value='zh'>简体中文</option>
              <option value='en'>English</option>
            </select>
          </div>
        </div>

        <div class="border-t border-dark-100 pt-6">
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-gear text-primary mr-2"></i>
            {{ t('setting.system-setting.title') }}
          </h3>
          <div class="pl-6 space-y-4">
            <label class="flex items-center justify-between">
              <span class="text-sm">{{ t('setting.system-setting.0') }}</span>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="settings.startOnBoot" class="sr-only peer">
                <div
                  class="w-11 h-6 bg-dark-100 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary">
                </div>
              </label>
            </label>
          </div>
        </div>

        <div class="border-t border-dark-100 pt-6">
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-trash text-primary mr-2"></i>
            {{ t('setting.data-management.title') }}
          </h3>
          <div class="pl-6 space-y-4">
            <button
              class="w-full text-left px-3 py-2 text-sm rounded bg-dark-300 hover:bg-dark-100 transition-colors flex justify-between items-center cursor-pointer">
              <span>{{ t('setting.data-management.0') }}</span>
              <i class="fa fa-arrow-right text-light-300"></i>
            </button>
          </div>
        </div>

        <div class="pt-4 border-t border-dark-100 flex justify-end">
          <button @click="saveSettings"
            class="px-4 py-2 text-sm rounded bg-primary hover:bg-primary/90 transition-colors cursor-pointer">
            {{ t('setting.save') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>