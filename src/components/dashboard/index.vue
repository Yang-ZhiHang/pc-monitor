<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { Chart, registerables } from 'chart.js';
import { invoke } from '@tauri-apps/api/core';
import { chartTitle } from '@/constants/dashboard';
import DoughnutChart from './chart/DoughnutChart.vue';
import BarChart from './chart/BarChart.vue';
import CompareCard from './CompareCard.vue';
import { getCurrentDate, getWeekDataAvg, getWeekDataSum } from '@/utils/date';
import { format_seconds } from '@/utils/format';
import { Mode } from '@/types/date';

// register all components of Chart.js
Chart.register(...registerables);

// ===== doughnut chart

let doughnutData = ref<Object | null>(null);
let doughnutOptions = ref<Object | null>(null);

function updateDoughnutData(dataset: Record<string, number>, k: number = 4) {
  if (dataset.length === 0) return;
  doughnutData.value = {
    labels: Object.keys(dataset).slice(0, k),
    datasets: [{
      label: "时长",
      data: Object.values(dataset).slice(0, k),
      borderWidth: 0,
      hoverOffset: 3
    }]
  };
}

function updateDoughnutOptions() {
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

const appUsage = ref<Record<string, number> | null>(null);

async function fetchRawAppUsage(days: number = 0): Promise<Record<string, Record<string, number>>> {
  return await invoke<Record<string, Record<string, number>>>("get_app_usage_duration_last_n_days", { n: days })
}

fetchRawAppUsage(1).then(_ => {
  const today = _[getCurrentDate(0)] || {};
  const yest = _[getCurrentDate(1)] || {};
  appUsage.value = Object.fromEntries(
    Object.entries(today).sort(([, a], [, b]) => b - a)
  );
  console.log("[appUsage] App usage duration:", appUsage.value);

  // Update doughnut chart
  updateDoughnutData(appUsage.value, 4);
  updateDoughnutOptions();

  // Obtain the length of appUsage
  cmpCardData.value[1][0] = Object.keys(today).length;
  cmpCardData.value[1][1] = Object.keys(yest).length;
  appUsageReady.value = true;
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});

const appUsageAvailable = () => {
  return appUsage.value !== null && Object.keys(appUsage.value).length > 0;
}

// ========== bar chart

let barData = ref<Object | null>(null);
let barOptions = ref<Object | null>(null);

import { getWeekData, getWeekLabelKey, getMonthWeekData, getMonthWeekLabelKey, getYearMonthData, getYearMonthLabelKey } from '@/utils/date';

function updateBarData(dataset: Record<string, number>, p: Period) {
  let labels: string[] = [];
  let data: number[] = [];
  switch (p) {
    case Period.WEEKLY:
      labels = getWeekLabelKey().map(label => t(label));
      data = getWeekData(dataset);
      break;
    case Period.MONTHLY:
      labels = getMonthWeekLabelKey().map(label => t(label));
      data = getMonthWeekData(dataset);
      break;
    case Period.YEARLY:
      labels = getYearMonthLabelKey().map(label => t(label));
      data = getYearMonthData(dataset);
      break;
  }
  barData.value = {
    labels,
    datasets: [
      {
        label: '时长',
        data,
        backgroundColor: 'rgba(59, 130, 246, 0.5)',
        borderColor: 'rgba(59, 130, 246, 0.8)',
        fill: false,
        cubicInterpolationMode: 'monotone',
        tension: 0.4,
        pointStyle: 'circle',
        pointRadius: 8,
        pointHoverRadius: 16
      }
    ]
  };
}

function updateBarOptions(dataset: Record<string, number>) {
  const maxVal = Math.max(...Object.values(dataset));
  const useHour = maxVal >= 3600;
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
            return useHour ? `${(value / 3600).toFixed(1)} h` : `${value}m`;
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

async function fetchRawDailyUsage(days: number = 0): Promise<Record<string, number>> {
  return await invoke<Record<string, number>>("get_daily_usage_duration_last_n_days", { n: days })
}

fetchRawDailyUsage(365).then(_ => {
  const result = Object.fromEntries(
    Object.keys(_).sort().map(key => [key, (_)[key]])
  );
  dailyUsage.value = result;
  console.log("[dailyUsage] Daily usage duration:", result);

  updateBarData(result, Period.WEEKLY);
  updateBarOptions(result);

  cmpCardData.value[0] = getCmpDays(dailyUsage.value ?? {});
  dailyUsageReady.value = true;
}).catch((error) => {
  console.error("Error fetching app usage duration:", error);
});

// ===== period

import { periodText } from '@/constants/dashboard';
import { Period } from '@/types/dashboard';
const period = ref<Period>(Period.WEEKLY);

watch(period, (newPeriod) => { updateBarData(dailyUsage.value ?? {}, newPeriod) });

// ===== cmp card

import { getCmpDays } from '@/utils/date';
import { compareCardInfo } from '../../constants/dashboard';

const cmpCardData = ref<Array<[number, number]>>([[0, 0], [0, 0]]);
const appUsageReady = ref<Boolean>(false);
const dailyUsageReady = ref<Boolean>(false);

// ===== refresh data

import { bus } from '@/utils/bus';

function fetchData() {
  fetchRawAppUsage(1).then(_ => {
    const today = _[getCurrentDate(0)] || {};
    const yest = _[getCurrentDate(1)] || {};
    appUsage.value = Object.fromEntries(
      Object.entries(today).sort(([, a], [, b]) => b - a)
    );
    console.log("[appUsage] App usage duration:", appUsage.value);

    // Update doughnut chart
    updateDoughnutData(appUsage.value, 4);
    updateDoughnutOptions();

    // Obtain the length of appUsage
    cmpCardData.value[1][0] = Object.keys(today).length;
    cmpCardData.value[1][1] = Object.keys(yest).length;
    appUsageReady.value = true;
  }).catch((error) => {
    console.error("Error fetching app usage duration:", error);
  });

  fetchRawDailyUsage(365).then(_ => {
    const result = Object.fromEntries(
      Object.keys(_).sort().map(key => [key, (_)[key]])
    );
    dailyUsage.value = result;
    console.log("[dailyUsage] Daily usage duration:", result);

    updateBarData(result, Period.WEEKLY);
    updateBarOptions(result);

    cmpCardData.value[0] = getCmpDays(dailyUsage.value ?? {});
    dailyUsageReady.value = true;
  }).catch((error) => {
    console.error("Error fetching app usage duration:", error);
  });
}

onMounted(() => {
  bus.on('refresh-dashboard', fetchData);
})
onUnmounted(() => {
  bus.off('refresh-dashboard', fetchData);
})
</script>

<template>
  <div class="h-full p-6 animate-fade overflow-auto">
    <!-- 概览卡片 -->
    <div class="grid grid-cols-3 gap-3 mb-3" v-if="appUsageReady && dailyUsageReady">
      <CompareCard :title="compareCardInfo[0].title" :formattedData="format_seconds(getWeekDataSum(dailyUsage!))"
        :cmpData="[getWeekDataSum(dailyUsage!), getWeekDataSum(dailyUsage!, Mode.SUB, 1)]" :icon="compareCardInfo[0].icon"
        :bgColor="compareCardInfo[0].bgColor" :cmpText="compareCardInfo[0].cmpText" />
      <CompareCard :title="compareCardInfo[1].title"
        :formattedData="`${Object.keys(appUsage!).length} ${t('dashboard.cmp-card.1.unit')}`"
        :cmpData="[cmpCardData[1][0], cmpCardData[1][1]]" :icon="compareCardInfo[1].icon"
        :bgColor="compareCardInfo[1].bgColor" :cmpText="compareCardInfo[1].cmpText" />
      <CompareCard :title="compareCardInfo[2].title" :formattedData="format_seconds(getWeekDataAvg(dailyUsage!))"
        :cmpData="[getWeekDataAvg(dailyUsage!), getWeekDataAvg(dailyUsage!, Mode.SUB, 1)]" :icon="compareCardInfo[2].icon"
        :bgColor="compareCardInfo[2].bgColor" :cmpText="compareCardInfo[2].cmpText" />
    </div>

    <!-- Chart Area -->
    <div class="grid grid-cols-3 gap-3 text-light-300">
      <div class="bg-dark-200 rounded-md p-5 card-shadow col-span-2 flex flex-col">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold">{{ t(chartTitle.dailyUsage) }}</h3>
          <div class="flex space-x-2">
            <button v-for="(label, key) in periodText" :key="key"
              class="px-2 py-1 text-xs rounded transition-colors font-bold" @click="period = key" :class="[
                period === key
                  ? 'bg-primary/20 text-primary cursor-default'
                  : 'bg-dark-100 hover:bg-dark-100/70 cursor-pointer'
              ]">{{ t(label) }}</button>
          </div>
        </div>
        <div class="flex-1 flex items-center justify-center">
          <BarChart v-if="barData && barOptions" :data="barData" :options="barOptions" />
        </div>
      </div>

      <div class="bg-dark-200 rounded-md p-5 card-shadow flex flex-col">
        <h3 class="font-semibold mb-4">{{ t(chartTitle.appUsage) }}</h3>
        <div class="flex-1 flex items-center justify-center">
          <DoughnutChart v-if="appUsageAvailable()" :data="doughnutData" :options="doughnutOptions" />
          <div v-else class="w-full h-full flex items-center justify-center">
            <p class="text-light-400">{{ t('dashboard.no-data') }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>