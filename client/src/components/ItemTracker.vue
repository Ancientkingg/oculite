<script lang="ts" setup>
import { reactive, ref, computed, watch } from 'vue';
import { useLayout } from '@/layout/composables/layout';

export interface Props {
    name: string;
    priceData: number[];
    currency: string;
    icon: string;
}

const { layoutConfig } = useLayout();
let documentStyle = getComputedStyle(document.documentElement);
const setColorOptions = () => {
    documentStyle = getComputedStyle(document.documentElement);
};

const props = withDefaults(defineProps<Props>(), {
    name: () => 'Toyota Avensis Remmen',
    priceData: () => [65, 59, 80, 81, 56, 55, 40],
    currency: () => '€',
    icon: () => 'data:image/png;base64,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'
});

const latestPrice = computed(() => {
    return props.priceData[props.priceData.length - 1].toFixed(2);
});

const priceChange = computed(() => {
    const latestPrice = Math.round(props.priceData[props.priceData.length - 1] * 100.0) / 100.0;
    const previousPrice = Math.round(props.priceData[props.priceData.length - 2] * 100.0) / 100.0;

    const percentageChange = Math.round((latestPrice - previousPrice) / previousPrice * 100.0);
    const signSymbol = percentageChange > 0 ? '+' : '';
    
    
    return `${signSymbol}${percentageChange}`
})

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

                let backgroundColor = documentStyle.getPropertyValue('--primary-800');

                const opacities = ['50', 'B2'];

                const {
                    ctx,
                    chartArea: { top, bottom },
                } = context.chart;
                const gradient = ctx.createLinearGradient(0, top, 0, bottom);

                gradient.addColorStop(1, backgroundColor + opacities[0]);
                // gradient.addColorStop(0, (backgroundColor + opacities[1]));

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
            <div class="card-details">
                <div class="card-divider" />
                <img
                    :src="icon"
                    height="100"
                    >
                <div class="card-price-details">
                    <h4 :class="`${priceChange[0] !== '-' ? 'text-green-500' : 'text-red-500'} text-center h-full mt-auto`">
                        {{ currency }} {{ latestPrice }}
                    </h4>
                    <div class="card-price-divider" />
                    <h6 class="card-price-percentage">
                        ({{ priceChange }}%)
                    </h6>
                </div>
            </div>

            <!-- <Chart
                class="w-3"
                type="line"
                :data="chartData"
                :options="chartOptions"
                /> -->
        </div>
    </div>
</template>
