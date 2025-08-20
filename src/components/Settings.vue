<script setup>
import { ref } from 'vue';

// 语言设置
const selectedLanguage = ref('zh-CN');

// 应用设置
const settings = ref({
  autoTrackApps: true,
  trackWebsites: true,
  startOnBoot: false
});

// 保存设置
const saveSettings = () => {
  // 这里可以添加保存设置的逻辑
  console.log('保存设置:', {
    language: selectedLanguage.value,
    ...settings.value
  });

  // 可以添加保存成功提示
  alert('设置已保存');
};

// 定义组件 emits
defineEmits(['switch-view']);
</script>

<template>
  <div class="min-h-min p-6 animate-fade">
    <div class="bg-dark-200 rounded-xl p-6 card-shadow h-full max-w-2xl mx-auto">
      <h2 class="text-xl font-semibold mb-6 text-light-300">设置</h2>

      <div class="space-y-6 text-light-300">
        <div>
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-globe text-primary mr-2"></i>
            语言设置
          </h3>
          <div class="pl-6">
            <label class="block text-light-300 text-sm mb-2">应用语言</label>
            <select v-model="selectedLanguage"
              class="w-full bg-dark-300 border border-dark-100 rounded-md mr-3 pl-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary cursor-pointer">
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
              <option value="ja-JP">日本語</option>
              <option value="ko-KR">한국어</option>
            </select>
          </div>
        </div>

        <div class="border-t border-dark-100 pt-6">
          <h3 class="font-medium mb-4 flex items-center">
            <i class="fa fa-gear text-primary mr-2"></i>
            系统设置
          </h3>
          <div class="pl-6 space-y-4">
            <label class="flex items-center justify-between">
              <span class="text-sm">开机自动启动</span>
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
            数据管理
          </h3>
          <div class="pl-6 space-y-4">
            <button
              class="w-full text-left px-3 py-2 text-sm rounded bg-dark-300 hover:bg-dark-100 transition-colors flex justify-between items-center cursor-pointer">
              <span>清除所有统计数据</span>
              <i class="fa fa-arrow-right text-light-300"></i>
            </button>

            <button
              class="w-full text-left px-3 py-2 text-sm rounded bg-dark-300 hover:bg-dark-100 transition-colors flex justify-between items-center cursor-pointer">
              <span>备份数据</span>
              <i class="fa fa-arrow-right text-light-300"></i>
            </button>
          </div>
        </div>

        <div class="pt-4 border-t border-dark-100 flex justify-end">
          <button @click="$emit('switch-view', 'stats')"
            class="px-4 py-2 mr-3 text-sm rounded border border-dark-100 hover:bg-dark-100 transition-colors cursor-pointer">
            取消
          </button>
          <button @click="saveSettings"
            class="px-4 py-2 text-sm rounded bg-primary hover:bg-primary/90 transition-colors cursor-pointer">
            保存设置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>