<script lang="ts" setup>
import { ref, toRefs, PropType } from 'vue';

const props = defineProps({
    title: {
        type: String,
        required: true
    },
    formattedData: {
        type: String,
        required: true
    },
    cmpData: {
        type: Object as PropType<[number, number]>,
        required: true
    },
    cmpText: {
        type: String,
        required: true,
    },
    icon: {
        type: String,
        required: true
    },
    bgColor: {
        type: String,
        required: true
    }
});

const { title, formattedData, cmpData, icon, bgColor } = toRefs(props);

const sign = ref<Boolean>(cmpData.value[0] - cmpData.value[1] >= 0);

const percent = ref<number>(Math.abs(Math.round((cmpData.value[0] - cmpData.value[1]) / cmpData.value[1] * 100)));

</script>

<template>
    <div class="bg-dark-200 text-light-300 rounded-md p-5 card-shadow hover-lift">
        <div class="flex justify-between items-start mb-4">
            <div class="">
                <p class="text-sm">{{ title }}</p>
                <h3 class="text-2xl font-bold mt-1">{{ formattedData }}</h3>
            </div>
            <div class="w-10 h-10 rounded-lg flex items-center justify-center" :class="bgColor">
                <i class="text-current" :class="`fa fa-${icon}`"></i>
            </div>
        </div>
        <div class="flex items-center text-sm">
            <span class="flex items-center" :class="sign ? 'text-secondary' : 'text-red-400'">
                <i class="mr-1 text-current" :class="sign ? 'fa fa-arrow-up' : 'fa fa-arrow-down'"></i>
                {{ percent }}%
            </span>
            <span class="ml-2">{{ cmpText }}</span>
        </div>
    </div>
</template>