<script lang="ts" setup>
import { useI18n } from "vue-i18n";
const { t } = useI18n();

import { ref } from 'vue';
import Dashboard from './components/dashboard/index.vue';
import ExportReport from './components/ExportReport.vue';
import Settings from './components/Settings.vue';
import WindowCtrls from "./components/WindowCtrls.vue";
import { navItems } from './constants/nav';

const currentRoute = ref<string>('dashboard');
const showExportSuccess = ref<boolean>(false);

const switchView = (view: string) => {
  currentRoute.value = view;
};
</script>

<template>
  <div class="flex h-screen bg-dark-300">
    <!-- Navigation Bar -->
    <div class="flex flex-col items-center justify-items-start px-3 py-3 w-50 bg-dark-400 border-b border-dark-100 z-10">
      <div class="flex items-center justify-around w-full py-2">
        <div class="w-8 h-8 rounded-md bg-primary flex items-center justify-center">
          <i class="fa fa-bar-chart text-white"></i>
        </div>
        <h1 class="text-xl font-semibold">{{ t('title') }}</h1>
      </div>

      <div class="flex flex-col w-full overflow-auto">
        <button v-for="(item, index) in navItems" :key="index" @click="switchView(item.route)"
          class="flex items-center mr-0 mt-4 px-3 py-1.5 rounded text-lg hover:bg-dark-100 transition-colors cursor-pointer"
          :class="{ 'bg-primary/20 text-primary': currentRoute === item.route }">
          <i :class="`fa fa-${item.icon} mr-2 flex! items-center h-full`"></i>
          <span class="flex-1 font-bold" >{{ t(item.label) }}</span>
        </button>
      </div>
    </div>

    <!-- Main Area - Component Switching -->
    <main class="flex-1 overflow-hidden relative">
      <WindowCtrls />
      <Dashboard v-if="currentRoute === `${navItems[0].route}`" />
      <ExportReport v-if="currentRoute === `${navItems[1].route}`" @export-success="showExportSuccess = true" />
      <Settings v-if="currentRoute === `${navItems[2].route}`" />
    </main>

    <!-- Export Success Notification -->
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