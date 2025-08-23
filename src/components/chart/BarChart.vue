<script setup>
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

defineProps({
    data: {
        type: Object,
        required: true
    },
    options: {
        type: Object,
        default: () => { }
    }
})
</script>

<template>
    <Bar :data="data" :options="options" />
</template>
