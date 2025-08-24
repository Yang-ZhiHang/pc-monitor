<script lang="ts" setup>
import { ref, watch } from 'vue';
import { Chart, registerables } from 'chart.js';
import { invoke } from '@tauri-apps/api/core';
import { chartTitle } from '@/constants/dashboard';
import DoughnutChart from './chart/DoughnutChart.vue';
import BarChart from './chart/BarChart.vue';

// register all components of Chart.js
Chart.register(...registerables);

let doughnutData = ref<Object | null>(null);
let doughnutOptions = ref<Object | null>(null);

function initDoughnutData(dataset: Record<string, number>, k: number = 4) {
  doughnutData.value = {
    labels: Object.keys(dataset).slice(0, k),
    datasets: [{
      label: "使用时长",
      data: Object.values(dataset).slice(0, k),
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
  const result = Object.fromEntries(
    Object
      .entries(_ as Record<string, number>)
      .sort(([, a], [, b]) => b - a)
  );
  console.log("App usage duration:", result);
  initDoughnutData(result, 4);
  initDoughnutOptions();
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});

// ========== end of doughnut chart

let barData = ref<Object | null>(null);
let barOptions = ref<Object | null>(null);

import dayjs from 'dayjs';

import { getWeekData, getWeekLabels, getMonthWeekData, getMonthWeekLabels, getYearMonthData, getYearMonthLabels } from '@/utils/date';

function initBarData(dataset: Record<string, number>, p: PeriodType) {
  const today = dayjs();
  let labels: string[] = [];
  let data: number[] = [];
  switch (p) {
    case Period.WEEKLY:
      labels = getWeekLabels();
      data = getWeekData(dataset, today);
      break;
    case Period.MONTHLY:
      labels = getMonthWeekLabels(today);
      data = getMonthWeekData(dataset, today);
      break;
    case Period.YEARLY:
      labels = getYearMonthLabels();
      data = getYearMonthData(dataset, today);
      break;
  }

  barData.value = {
    labels,
    datasets: [{
      label: '使用时长 (小时)',
      data,
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
    scales: {
      y: {
        beginAtZero: true,
        grid: {
          color: 'rgba(255, 255, 255, 0.05)'
        },
        ticks: {
          color: 'rgba(255, 255, 255, 0.7)',
          // seconds to hours
          callback: function (value: number) {
            return Math.round(value / 3600);
          }
        },
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

const dailyUsage = ref<Record<string, number> | null>(null);
invoke("get_recall_usage_duration_rs", { n: 365 }).then(_ => {
  const result = Object.fromEntries(
    Object
      .keys(_ as Record<string, number>)
      .sort()
      .map(key => [key, (_ as Record<string, number>)[key]])
  );
  dailyUsage.value = result;
  console.log("Daily usage duration:", result);
  initBarData(result, Period.WEEKLY);
  initBarOptions();
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});

import { periodText, Period, type PeriodType } from '@/constants/dashboard';
const period = ref<PeriodType>(Period.WEEKLY);

watch(() => period.value, (newPeriod) => { initBarData(dailyUsage.value ?? {}, newPeriod) });

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
            <button v-for="(label, key) in periodText" :key="key" class="px-2 py-1 text-xs rounded transition-colors"
              @click="period = key" :class="[
                period === key
                  ? 'bg-primary/20 text-primary cursor-default'
                  : 'bg-dark-100 hover:bg-dark-100/70 cursor-pointer'
              ]">{{ label }}</button>
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