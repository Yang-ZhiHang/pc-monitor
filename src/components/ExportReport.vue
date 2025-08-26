<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
import { ref } from 'vue';
import { ElDatePicker } from 'element-plus';

// 日期状态
const startDate = ref('2023-06-01');
const endDate = ref('2023-06-30');

// 导出选项
const exportOptions = ref({
  appUsage: true,
  webHistory: true,
  durationStats: true,
});

// 导出格式
const exportFormat = ref('html');

// 处理导出
const handleExport = () => {
  // 这里可以添加导出逻辑
  console.log('导出数据:', {
    startDate: startDate.value,
    endDate: endDate.value,
    options: exportOptions.value,
    format: exportFormat.value
  });
};
</script>

<template>
  <div class="min-h-min p-6 animate-fade overflow-auto">
    <div class="h-full w-full min-w-xl mx-auto bg-dark-200 rounded-md p-6 card-shadow">
      <h2 class="text-xl text-light-300 font-semibold mb-6">{{ t('export.title') }}</h2>

      <div class="space-y-6">
        <div>
          <label class="block text-light-300 text-sm mb-2">{{ t('export.date-range.title') }}</label>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="text-xs text-light-300 mb-1 block">{{ t('export.date-range.start') }}</label>
              <el-date-picker
                v-model="startDate"
                type="date"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                class="w-full"
                popper-class="bg-dark-200 text-light-400"
              />
            </div>
            <div>
              <label class="text-xs text-light-300 mb-1 block">{{ t('export.date-range.end') }}</label>
              <el-date-picker
                v-model="endDate"
                type="date"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                class="w-full"
                popper-class="bg-dark-200 text-light-400"
              />
            </div>
          </div>
        </div>

        <div>
          <label class="block text-light-300 text-sm mb-2">{{ t('export.export-ctx.title') }}</label>
          <div class="space-y-2 text-light-300">
            <label class="flex items-center hover:bg-dark-100">
              <input type="checkbox" v-model="exportOptions.appUsage"
                class="rounded bg-dark-300 border-dark-100 text-primary focus:ring-primary cursor-pointer">
              <span class="ml-2 text-sm">{{ t('export.export-ctx.0') }}</span>
            </label>
            <label class="flex items-center hover:bg-dark-100">
              <input type="checkbox" v-model="exportOptions.durationStats"
                class="rounded bg-dark-300 border-dark-100 text-primary focus:ring-primary cursor-pointer">
              <span class="ml-2 text-sm">{{ t('export.export-ctx.1') }}</span>
            </label>
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