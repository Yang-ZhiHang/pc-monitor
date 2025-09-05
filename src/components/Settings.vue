<script lang="ts" setup>
import { watch } from 'vue';
import { ElSelect, ElOption } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';

import { useI18n } from 'vue-i18n';
import { useSettingStore } from '@/stores/setting';
const { t } = useI18n();
const settingStore = useSettingStore();

watch(() => settingStore.startOnBoot, (newVal) => {
  invoke<void>('set_start_on_boot_rs', { enable: newVal });
});
</script>

<template>
  <div class="min-h-min p-6 animate-fade overflow-auto">
    <div class="h-full w-full min-w-xl mx-auto bg-dark-200 rounded-md p-6 card-shadow">
      <h2 class="text-xl font-semibold mb-6 text-light-300">{{ t('setting.title') }}</h2>

      <div class="space-y-6 text-light-300">
        <div>
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-globe text-primary mr-2"></i>
            {{ t('setting.language.title') }}
          </h3>
          <div class="pl-6">
            <el-select v-model="settingStore.lang" class="w-full" size="large">
              <el-option label="简体中文" value="zh" @click="settingStore.switchLang('zh')" />
              <el-option label="English" value="en" @click="settingStore.switchLang('en')" />
            </el-select>
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
                <input type="checkbox" v-model="settingStore.startOnBoot" class="sr-only peer">
                <div
                  class="w-11 h-6 bg-dark-100 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary">
                </div>
              </label>
            </label>
            <label class="flex items-center justify-between">
              <span class="text-sm">{{ t('setting.system-setting.1') }}</span>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="settingStore.minToTray" class="sr-only peer">
                <div
                  class="w-11 h-6 bg-dark-100 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary">
                </div>
              </label>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>