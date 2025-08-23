<script lang="ts" setup>
import { ref } from 'vue';
import { Chart, registerables } from 'chart.js';
import { invoke } from '@tauri-apps/api/core';
import { ignoreAppList, chartTitle } from '@/constants/dashboard';
import DoughnutChart from './chart/DoughnutChart.vue';
import BarChart from './chart/BarChart.vue';

console.log("setup executed");

// register all components of Chart.js
Chart.register(...registerables);

let doughnutData = ref<Object | null>(null);
let doughnutOptions = ref<Object | null>(null);

function initDoughnutData(label: string[], data: number[], k: number = 4) {
  doughnutData.value = {
    labels: label.slice(0, k),
    datasets: [{
      label: "使用时长",
      data: data.slice(0, k),
      borderWidth: 0,
      hoverOffset: 3
    }]
  };
}

function initDoughnutOptions() {
  doughnutOptions.value = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'bottom',
        labels: {
          color: 'rgba(255, 255, 255, 0.7)',
          padding: 15,
          boxWidth: 10,
          boxHeight: 10,
          usePointStyle: true,
          pointStyle: 'circle'
        }
      },
      tooltip: {
        callbacks: {
          label: function (context: any) {
            const value = context.dataset.data[context.dataIndex];
            return `${context.dataset.label}: ${format_seconds(value)}`;
          }
        }
      }
    },
    cutout: '70%'
  };
}

function format_seconds(sec: number) {
  let h = Math.floor(sec / 3600);
  let m = Math.floor((sec % 3600) / 60);
  let s = Math.floor(sec % 60);
  if (h == 0 && m == 0) {
    return `${s}s`;
  } else if (h == 0) {
    return `${m}m${s}s`;
  }
  return `${h}h ${m}m ${s}s`;
}

invoke("get_app_usage_duration_rs").then(_ => {
  let result = Object.entries(_ as Record<string, number>).sort(([, a], [, b]) => b - a);
  let label: string[] = [];
  let time: number[] = [];
  for (const [key, val] of result) {
    if (ignoreAppList.includes(key)) continue;
    label.push(key);
    time.push(val);
  }
  initDoughnutData(label, time, 5);
  initDoughnutOptions();
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});

let barData = ref<Object | null>(null);
let barOptions = ref<Object | null>(null);

function initBarData() {
  barData.value = {
    labels: ['1日', '5日', '10日', '15日', '20日', '25日', '30日'],
    datasets: [{
      label: '使用时长 (小时)',
      data: [4.5, 5.2, 3.8, 6.1, 4.9, 5.5, 5.3],
      backgroundColor: 'rgba(59, 130, 246, 0.7)',
      borderRadius: 6,
      doughnutPercentage: 0.6,
    }]
  };
}

function initBarOptions() {
  barOptions.value = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false
      }
    },
    scales: {
      y: {
        beginAtZero: true,
        grid: {
          color: 'rgba(255, 255, 255, 0.05)'
        },
        ticks: {
          color: 'rgba(255, 255, 255, 0.7)'
        }
      },
      x: {
        grid: {
          display: false
        },
        ticks: {
          color: 'rgba(255, 255, 255, 0.7)'
        }
      }
    }
  }
}
invoke("get_daily_usage_duration_rs").then(_ => {
  let result = _ as number;
  console.log("Daily usage duration:", result);
  let h = Math.round(result / 60 / 60);
  let m = Math.round(result / 60 % 60);
  let s = Math.round(result % 60);
  console.log(`${h}:${m}:${s}`);
  initBarData();
  initBarOptions();
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});


</script>

<template>
  <div class="h-full p-6 animate-fade">
    <!-- 概览卡片 -->
    <div class="grid grid-cols-3 gap-6 mb-6">
      <div class="bg-dark-200 text-light-300 rounded-xl p-5 card-shadow hover-lift">
        <div class="flex justify-between items-start mb-4">
          <div class="">
            <p class="text-sm">今日电脑使用时长</p>
            <h3 class="text-2xl font-bold mt-1">5h 32m</h3>
          </div>
          <div class="w-10 h-10 rounded-lg bg-primary/20 flex items-center justify-center text-primary">
            <i class="fa-regular fa-clock text-current"></i>
          </div>
        </div>
        <div class="flex items-center text-sm">
          <span class="text-secondary flex items-center">
            <i class="fa fa-arrow-up mr-1 text-current"></i> 12%
          </span>
          <span class="ml-2">较昨日</span>
        </div>
      </div>

      <div class="bg-dark-200 text-light-300 rounded-xl p-5 card-shadow hover-lift">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm">打开应用数</p>
            <h3 class="text-2xl font-bold mt-1">18</h3>
          </div>
          <div class="w-10 h-10 rounded-lg bg-purple-500/20 flex items-center justify-center text-purple-400">
            <i class="fa fa-th-large text-current"></i>
          </div>
        </div>
        <div class="flex items-center text-sm">
          <span class="text-red-400 flex items-center">
            <i class="fa fa-arrow-down mr-1 text-current"></i> 3
          </span>
          <span class="ml-2">较昨日</span>
        </div>
      </div>
    </div>

    <!-- 图表区域 -->
    <div class="grid grid-cols-3 gap-6 h-[calc(100%-12rem)] text-light-300">
      <div class="bg-dark-200 rounded-xl p-5 card-shadow col-span-2 flex flex-col">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold">{{ chartTitle.dailyUsage }}</h3>
          <div class="flex space-x-2">
            <button
              class="px-2 py-1 text-xs rounded bg-dark-100 hover:bg-dark-100/70 transition-colors cursor-pointer">周</button>
            <button class="px-2 py-1 text-xs rounded bg-primary/20 text-primary cursor-pointer">月</button>
            <button
              class="px-2 py-1 text-xs rounded bg-dark-100 hover:bg-dark-100/70 transition-colors cursor-pointer">年</button>
          </div>
        </div>
        <div class="flex-1 flex items-center justify-center">
          <!-- <canvas id="dailyUsageChart" class="w-full h-full"></canvas> -->
          <BarChart v-if="barData && barOptions" :data="barData" :options="barOptions" />
        </div>
      </div>

      <div class="bg-dark-200 rounded-xl p-5 card-shadow flex flex-col">
        <h3 class="font-semibold mb-4">{{ chartTitle.appUsage }}</h3>
        <div class="flex-1 flex items-center justify-center">
          <!-- <canvas id="appUsageChart" class="w-full h-full max-w-[300px] max-h-[300px]"></canvas> -->
          <DoughnutChart v-if="doughnutData && doughnutOptions" :data="doughnutData" :options="doughnutOptions" />
        </div>
      </div>
    </div>
  </div>
</template>