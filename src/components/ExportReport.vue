<script lang="ts" setup>
import { ref } from 'vue';
import { ElDatePicker, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { ExportFormat } from '@/types/export';
import { exportOptions } from '@/constants/export';

import { useI18n } from 'vue-i18n';
import { ElConfigProvider } from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'
import { useSettingStore } from '@/stores/setting';

const { t } = useI18n();
const settingStore = useSettingStore();

const dateRange = ref<[string, string]>([
  new Date(new Date().setHours(0, 0, 0, 0)).toISOString().split('T')[0],
  new Date(new Date().setHours(23, 59, 59, 999)).toISOString().split('T')[0]
]);

const size = ref<'default' | 'large' | 'small'>('large')
const exportFormat = ref<ExportFormat>(ExportFormat.HTML);

const handleExport = () => {
  invoke<void>('export_report', {
    startDate: dateRange.value[0],
    endDate: dateRange.value[1],
    format: exportFormat.value
  }).catch(err => {
    ElMessage.error(`Export failed: ${err}`);
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

const getLastWeekRange = (): [string, string] => {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - 6);
  return [start.toISOString().split('T')[0], end.toISOString().split('T')[0]];
};

const getLastMonthRange = (): [string, string] => {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - 31);
  return [start.toISOString().split('T')[0], end.toISOString().split('T')[0]];
};
</script>

<template>
  <div class="min-h-min p-6 animate-fade overflow-auto">
    <div class="grid grid-cols-3 gap-3 mb-3 text-light-300">
      <div class="bg-dark-200 rounded-md p-5 card-shadow col-span-2 flex flex-col">
        <div class="flex items-center justify-start w-full pb-2">
          <div class="flex items-center justify-center w-6 h-6 mr-2 text-primary rounded-md ">
            <i class="fa-regular fa-calendar text-lg"></i>
          </div>
          <h2 class="text-lg font-semibold">{{ t('export.date-range.title') }}</h2>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <el-config-provider :locale="settingStore.lang == 'en' ? en : zhCn">
            <el-date-picker v-model="dateRange" type="daterange" :start-placeholder="t('export.date-range.start')"
              :end-placeholder="t('export.date-range.end')" format="YYYY-MM-DD" value-format="YYYY-MM-DD" class="w-full"
              :shortcuts="shortcuts" :disabled-date="disabledDate" :size="size"
              popper-class="bg-dark-200 text-light-400" />
          </el-config-provider>
        </div>
        <p class="text-xs pt-2 text-gray-400">{{ t('export.date-range.tip') }}</p>
        <div class="mt-2 space-x-3">
          <button @click="() => dateRange = getLastWeekRange()"
            class="px-2 py-1 text-sm rounded-md border border-gray-500 hover:border-primary transition-colors cursor-pointer">{{
              t('export.date-range.last-week') }}</button>
          <button @click="() => dateRange = getLastMonthRange()"
            class="px-2 py-1 text-sm rounded-md border border-gray-500 hover:border-primary transition-colors cursor-pointer">{{
              t('export.date-range.last-month') }}</button>
        </div>
      </div>
      <div class="bg-dark-200 rounded-md p-5 card-shadow col-span-1 flex flex-col">
        <h2 class="text-lg font-semibold">{{ t('export.save') }}</h2>
        <p class="text-xs pt-2 text-gray-400">{{ t('export.export.date-range') }}:
          {{ dateRange[0] }} to {{ dateRange[1] }}
        </p>
        <p class="text-xs pt-2 text-gray-400">{{ t('export.export.export-fmt') }}: {{ exportFormat }}</p>
        <div class="pt-4 border-dark-100 flex justify-end text-light-300">
          <button @click="handleExport"
            class="flex items-center justify-center w-full px-4 py-2 text-sm rounded bg-primary hover:bg-primary/90 transition-colors cursor-pointer">
            <i class="fa fa-download mr-2"></i>
            {{ t("export.save") }}
          </button>
        </div>
      </div>
    </div>
    <div class="grid grid-cols-3 gap-3 text-light-300">
      <div class="bg-dark-200 rounded-md p-5 card-shadow col-span-2 flex flex-col">
        <div class="flex items-center justify-start w-full pb-2">
          <div class="flex items-center justify-center w-6 h-6 mr-2 text-secondary rounded-md ">
            <i class="fa-solid fa-download text-lg"></i>
          </div>
          <h2 class="text-lg font-semibold">{{ t('export.export-fmt.title') }}</h2>
        </div>
        <div class="space-y-3">
          <label v-for="(option, key) in exportOptions" :key="key"
            class="flex items-center p-3 border border-dark-100 rounded-md hover:border-primary cursor-pointer transition-colors"
            :class="{ 'border-primary bg-primary/10': exportFormat === key }">
            <input type="radio" name="format" :value="key" v-model="exportFormat" class="sr-only">
            <i :class="[option.icon, 'mr-3']"></i>
            <div class="flex flex-col">
              <span class="text-sm">{{ t(option.title) }}</span>
              <span class="text-xs text-gray-400">{{ t(option.description) }}</span>
            </div>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>