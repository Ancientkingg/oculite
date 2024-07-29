<script lang="ts" setup>
import { reactive, ref, watch } from 'vue';
import { useLayout } from '@/layout/composables/layout';

export interface Props {
    name: string;
    priceData: any;
}

const { layoutConfig } = useLayout();
let documentStyle = getComputedStyle(document.documentElement);
const setColorOptions = () => {
    documentStyle = getComputedStyle(document.documentElement);
};

const props = withDefaults(defineProps<Props>(), {
    name: 'Name',
    priceData: [65, 59, 80, 81, 56, 55, 40],
});

const chartData = reactive({
    labels: props.priceData.map((_, index) => index),
    datasets: [
        {
            label: 'Price',
            data: props.priceData,
            fill: true,
            tension: 0.4,
            backgroundColor: (context, _scales) => {
                if (!context.chart.chartArea) return;

                const backgroundColor = documentStyle.getPropertyValue('--primary-500');

                const opacities = ['00', 'E6'];

                const { ctx, chartArea: { top, bottom } } = context.chart;
                const gradient = ctx.createLinearGradient(0, top, 0, bottom);

                gradient.addColorStop(1, (backgroundColor + opacities[0]));
                gradient.addColorStop(0, (backgroundColor + opacities[1]));

                return gradient;
            },
            borderColor: documentStyle.getPropertyValue('--primary-500'),
        },
    ],
});

const chartOptions = ref({
    plugins: {
        legend: {
            display: false,
        },
    },
    elements: {
        point: {
            radius: 0,
        },
    },
    scales: {
        x: {
            ticks: {
                display: false,
            },
            grid: {
                display: false,
            },
        },
        y: {
            ticks: {
                display: false,
            },
            grid: {
                display: false,
            },
        },
    },
});

watch(
    layoutConfig.theme,
    () => {
        setColorOptions();
    },
    { immediate: true },
);
</script>

<template>
    <div class="col-12 lg:col-6 xl:col-4">
        <div class="card card-clickable">
            <h5>{{ name }}</h5>
            <Chart
                class=""
                type="line"
                :data="chartData"
                :options="chartOptions"
                />
        </div>
    </div>
</template>
