<script lang="ts" setup>
import { ref } from 'vue';
import Dashboard from './components/Dashboard.vue';
import ExportReport from './components/ExportReport.vue';
import Settings from './components/Settings.vue';
import { navItems, title } from './constants/nav';

const currentRoute = ref<string>('dashboard');
const showExportSuccess = ref<boolean>(false);

const switchView = (view: string) => {
  currentRoute.value = view;
};
</script>

<template>
  <div class="flex flex-col h-screen bg-dark-300">
    <!-- 顶部导航栏 -->
    <header class="bg-dark-400 border-b border-dark-100 px-3 py-3 flex items-center justify-between z-10">
      <div class="flex items-center space-x-3">
        <div class="w-8 h-8 rounded-md bg-primary flex items-center justify-center">
          <i class="fa fa-bar-chart text-white"></i>
        </div>
        <h1 class="text-xl font-semibold">{{ title }}</h1>
      </div>

      <div class="flex items-center space-x-4">
        <button v-for="(item, index) in navItems" :key="index" @click="switchView(item.route)"
          class="px-3 py-1.5 rounded text-sm hover:bg-dark-100 transition-colors flex items-center cursor-pointer"
          :class="{ 'bg-primary/20 text-primary': currentRoute === item.route }">
          <i :class="`fa fa-${item.icon} mr-2`"></i>
          <span>{{ item.label }}</span>
        </button>

        <div class="w-px h-6 bg-dark-100 mx-1"></div>

        <button
          class="w-8 h-8 rounded-full flex items-center justify-center hover:bg-dark-100 transition-colors cursor-pointer">
          <i class="fa fa-question-circle"></i>
        </button>
      </div>
    </header>

    <!-- 主内容区域 - 组件切换 -->
    <main class="flex-1 overflow-auto relative">
      <Dashboard v-if="currentRoute === `${navItems[0].route}`" />
      <ExportReport v-if="currentRoute === `${navItems[1].route}`" @export-success="showExportSuccess = true" />
      <Settings v-if="currentRoute === `${navItems[2].route}`" />
    </main>

    <!-- 底部状态栏 -->
    <footer class="bg-dark-400 border-t border-dark-100 px-6 py-2 text-xs flex justify-between items-center">
      <div>
        <span>版本 1.0.0</span>
      </div>
      <div>
        <span>上次更新: 今天 14:30</span>
      </div>
    </footer>

    <!-- 导出成功提示 -->
    <div v-if="showExportSuccess"
      class="fixed top-4 right-4 bg-dark-200 rounded-lg p-4 shadow-lg border-l-4 border-secondary animate-fade max-w-xs z-50">
      <div class="flex items-start">
        <div class="flex-shrink-0">
          <i class="fa fa-check-circle text-secondary text-xl"></i>
        </div>
        <div class="ml-3">
          <p class="text-sm font-medium">导出成功</p>
          <p class="text-xs mt-1">统计数据已成功导出，正在打开预览...</p>
        </div>
        <button @click="showExportSuccess = false" class="ml-auto flex-shrink-0">
          <i class="fa fa-times hover:text-light-100"></i>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
::-webkit-scrollbar {
  width: 0;
}
</style>