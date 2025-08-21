<script setup>
import { ref } from 'vue';

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

  // 触发导出成功事件
  $emit('export-success');
};

// 定义组件 emits
defineEmits(['export-success', 'switch-view']);
</script>

<template>
  <div class="min-h-min p-6 animate-fade">
    <div class="bg-dark-200 rounded-xl p-6 card-shadow h-full max-w-2xl mx-auto">
      <h2 class="text-xl text-light-300 font-semibold mb-6">导出统计数据</h2>

      <div class="space-y-6">
        <div>
          <label class="block text-light-300 text-sm mb-2">日期范围</label>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="text-xs text-light-300 mb-1 block">开始日期</label>
              <div class="relative">
                <input type="date" v-model="startDate"
                  class="w-full bg-dark-200 text-light-400 border border-dark-100 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary">
              </div>
            </div>
            <div>
              <label class="text-xs text-light-300 mb-1 block">结束日期</label>
              <div class="relative">
                <input type="date" v-model="endDate"
                  class="w-full bg-dark-200 text-light-400 border border-dark-100 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary">
              </div>
            </div>
          </div>
        </div>

        <div>
          <label class="block text-light-300 text-sm mb-2">导出内容</label>
          <div class="space-y-2 text-light-300">
            <label class="flex items-center">
              <input type="checkbox" v-model="exportOptions.appUsage"
                class="rounded bg-dark-300 border-dark-100 text-primary focus:ring-primary cursor-pointer">
              <span class="ml-2 text-sm">应用使用时间</span>
            </label>
            <label class="flex items-center">
              <input type="checkbox" v-model="exportOptions.durationStats"
                class="rounded bg-dark-300 border-dark-100 text-primary focus:ring-primary cursor-pointer">
              <span class="ml-2 text-sm">使用时长统计</span>
            </label>
          </div>
        </div>

        <div>
          <label class="block text-light-300 text-sm mb-2">导出格式</label>
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
          <button @click="$emit('switch-view', 'stats')"
            class="px-4 py-2 mr-3 text-sm rounded border border-dark-100 hover:bg-dark-100 transition-colors cursor-pointer">
            取消
          </button>
          <button @click="handleExport"
            class="px-4 py-2 text-sm rounded bg-primary hover:bg-primary/90 transition-colors flex items-center cursor-pointer">
            <i class="fa fa-download mr-2"></i>
            导出数据
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