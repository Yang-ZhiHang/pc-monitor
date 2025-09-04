<script lang="ts" setup>
import { ref } from 'vue';
import { ElDatePicker } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';

import { useI18n } from 'vue-i18n';
import { ElConfigProvider } from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'
import { useSettingStore } from '@/stores/setting';

const { t } = useI18n();
const settingStore = useSettingStore();

// [start, end]
const dateRange = ref<[string, string]>([
  new Date(new Date().setHours(0, 0, 0, 0)).toISOString().split('T')[0],
  new Date(new Date().setHours(23, 59, 59, 999)).toISOString().split('T')[0]
]);

const size = ref<'default' | 'large' | 'small'>('large')

// Default format
const exportFormat = ref('html');

const handleExport = () => {
  invoke<void>('export_report', {
    startDate: dateRange.value[0],
    endDate: dateRange.value[1],
    format: exportFormat.value
  })
};

const shortcuts = [
  {
    text: t('export.date-range.picker.today'),
    value: [new Date(), new Date()],
  },
  {
    text: t('export.date-range.picker.yesterday'),
    value: () => {
      const date = new Date()
      date.setTime(date.getTime() - 3600 * 1000 * 24)
      return [date, new Date()]
    },
  },
  {
    text: t('export.date-range.picker.last-week'),
    value: () => {
      const date = new Date()
      date.setTime(date.getTime() - 3600 * 1000 * 24 * 7)
      return [date, new Date()]
    },
  },
  {
    text: t('export.date-range.picker.last-month'),
    value: () => {
      const date = new Date()
      date.setTime(date.getTime() - 3600 * 1000 * 24 * 30)
      return [date, new Date()]
    },
  },
]

const disabledDate = (time: Date) => {
  return time.getTime() > Date.now()
}
</script>

<template>
  <div class="min-h-min p-6 animate-fade overflow-auto">
    <div class="h-full w-full min-w-xl mx-auto bg-dark-200 rounded-md p-6 card-shadow">
      <h2 class="text-xl text-light-300 font-semibold mb-6">{{ t('export.title') }}</h2>

      <div class="space-y-6">
        <div>
          <label class="block text-light-300 text-sm mb-2">{{ t('export.date-range.title') }}</label>
          <div class="grid grid-cols-2 gap-4">
            <el-config-provider :locale="settingStore.lang == 'en' ? en : zhCn">
                <el-date-picker v-model="dateRange" type="daterange" :start-placeholder="t('export.date-range.start')" :end-placeholder="t('export.date-range.end')"
                  format="YYYY-MM-DD" value-format="YYYY-MM-DD" class="w-full" :shortcuts="shortcuts"
                  :disabled-date="disabledDate" :size="size" popper-class="bg-dark-200 text-light-400" />
            </el-config-provider>
          </div>
        </div>

        <div>
          <label class="block text-light-300 text-sm mb-2">{{ t('export.export-fmt') }}</label>
          <div class="grid grid-cols-3 gap-3 text-light-300">
            <label
              class="flex flex-col items-center p-3 border border-dark-100 rounded-md hover:border-primary cursor-pointer transition-colors"
              :class="{ 'border-primary bg-primary/10': exportFormat === 'html' }">
              <input type="radio" name="format" value="html" v-model="exportFormat" class="sr-only">
              <i class="fa-brands fa-html5 text-orange-500 text-xl mb-1"></i>
              <span class="text-sm">HTML</span>
            </label>
            <label
              class="flex flex-col items-center p-3 border border-dark-100 rounded-md hover:border-primary cursor-pointer transition-colors"
              :class="{ 'border-primary bg-primary/10': exportFormat === 'csv' }">
              <input type="radio" name="format" value="csv" v-model="exportFormat" class="sr-only">
              <i class="fa fa-file-text text-green-500 text-xl mb-1"></i>
              <span class="text-sm">CSV</span>
            </label>
            <label
              class="flex flex-col items-center p-3 border border-dark-100 rounded-md hover:border-primary cursor-pointer transition-colors"
              :class="{ 'border-primary bg-primary/10': exportFormat === 'json' }">
              <input type="radio" name="format" value="json" v-model="exportFormat" class="sr-only">
              <i class="fa fa-code text-purple-500 text-xl mb-1"></i>
              <span class="text-sm">JSON</span>
            </label>
          </div>
        </div>

        <div class="pt-4 border-t border-dark-100 flex justify-end text-light-300">
          <button @click="handleExport"
            class="px-4 py-2 text-sm rounded bg-primary hover:bg-primary/90 transition-colors flex items-center cursor-pointer">
            <i class="fa fa-download mr-2"></i>
            {{ t("export.save") }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
input[type="date"]::-webkit-calendar-picker-indicator {
  filter: invert(1);
}
</style>