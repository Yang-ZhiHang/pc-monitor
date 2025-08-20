<template>
  <div class="h-full p-6 animate-fade">
    <!-- 概览卡片 -->
    <div class="grid grid-cols-3 gap-6 mb-6">
      <div class="bg-dark-200 text-light-300 rounded-xl p-5 card-shadow hover-lift">
        <div class="flex justify-between items-start mb-4">
          <div class="">
            <p class="text-sm">今日使用时长</p>
            <h3 class="text-2xl font-bold mt-1">5h 32m</h3>
          </div>
          <div class="w-10 h-10 rounded-lg bg-primary/20 flex items-center justify-center text-primary">
            <i class="fa fa-clock text-current"></i>
          </div>
        </div>
        <div class="flex items-center text-sm">
          <span class="text-secondary flex items-center">
            <i class="fa fa-arrow-up mr-1"></i> 12%
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
          <span class="text-secondary flex items-center">
            <i class="fa fa-arrow-up mr-1"></i> 3
          </span>
          <span class="ml-2">较昨日</span>
        </div>
      </div>
      
      <div class="bg-dark-200 text-light-300 rounded-xl p-5 card-shadow hover-lift">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm">productive 时间</p>
            <h3 class="text-2xl font-bold mt-1">3h 15m</h3>
          </div>
          <div class="w-10 h-10 rounded-lg bg-green-500/20 flex items-center justify-center text-green-400">
            <i class="fa fa-check-circle"></i>
          </div>
        </div>
        <div class="flex items-center text-sm">
          <span class="text-red-400 flex items-center">
            <i class="fa fa-arrow-down mr-1"></i> 5%
          </span>
          <span class="ml-2">较昨日</span>
        </div>
      </div>
    </div>
    
    <!-- 图表区域 -->
    <div class="grid grid-cols-3 gap-6 h-[calc(100%-12rem)] text-light-300">
      <div class="bg-dark-200 rounded-xl p-5 card-shadow col-span-2 flex flex-col">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold">每日使用时长</h3>
          <div class="flex space-x-2">
            <button class="px-2 py-1 text-xs rounded bg-dark-100 hover:bg-dark-100/70 transition-colors cursor-pointer">周</button>
            <button class="px-2 py-1 text-xs rounded bg-primary/20 text-primary cursor-pointer">月</button>
            <button class="px-2 py-1 text-xs rounded bg-dark-100 hover:bg-dark-100/70 transition-colors cursor-pointer">年</button>
          </div>
        </div>
        <div class="flex-1 flex items-center justify-center">
          <canvas id="dailyUsageChart" class="w-full h-full"></canvas>
        </div>
      </div>
      
      <div class="bg-dark-200 rounded-xl p-5 card-shadow flex flex-col">
        <h3 class="font-semibold mb-4">应用使用占比</h3>
        <div class="flex-1 flex items-center justify-center">
          <canvas id="appUsageChart" class="w-full h-full max-w-[200px] max-h-[200px]"></canvas>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, onUnmounted } from 'vue';
import { Chart, registerables } from 'chart.js';

// 注册Chart.js所有组件
Chart.register(...registerables);

// 存储图表实例，用于组件卸载时销毁
let dailyChart = null;
let appChart = null;

onMounted(() => {
  // 每日使用时长图表
  const dailyCtx = document.getElementById('dailyUsageChart').getContext('2d');
  dailyChart = new Chart(dailyCtx, {
    type: 'bar',
    data: {
      labels: ['1日', '5日', '10日', '15日', '20日', '25日', '30日'],
      datasets: [{
        label: '使用时长 (小时)',
        data: [4.5, 5.2, 3.8, 6.1, 4.9, 5.5, 5.3],
        backgroundColor: 'rgba(59, 130, 246, 0.7)',
        borderRadius: 6,
        barPercentage: 0.6,
      }]
    },
    options: {
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
  });
  
  // 应用使用占比图表
  const appCtx = document.getElementById('appUsageChart').getContext('2d');
  appChart = new Chart(appCtx, {
    type: 'doughnut',
    data: {
      labels: ['浏览器', '编辑器', '终端', '其他'],
      datasets: [{
        data: [40, 30, 15, 15],
        backgroundColor: [
          'rgba(59, 130, 246, 0.8)',
          'rgba(16, 185, 129, 0.8)',
          'rgba(139, 92, 246, 0.8)',
          'rgba(66, 70, 85, 0.8)'
        ],
        borderWidth: 0,
        hoverOffset: 4
      }]
    },
    options: {
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
        }
      },
      cutout: '70%'
    }
  });
});

// 组件卸载时销毁图表，避免内存泄漏
onUnmounted(() => {
  if (dailyChart) {
    dailyChart.destroy();
  }
  if (appChart) {
    appChart.destroy();
  }
});
</script>
    