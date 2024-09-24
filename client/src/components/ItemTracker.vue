<script lang="ts" setup>
import { reactive, ref, computed, watch, getCurrentScope, getCurrentInstance } from 'vue';
import { useLayout } from '@/layout/composables/layout';
import ItemTracker from '@/model/ItemTracker';
import { favorite as favoriteApi, unfavorite as unfavoriteApi } from '@/services/itemTrackerService'


const { layoutConfig } = useLayout();
let documentStyle = getComputedStyle(document.documentElement);
const setColorOptions = () => {
    documentStyle = getComputedStyle(document.documentElement);
};

const model = defineModel<ItemTracker>();

if (model.value === undefined) {
    throw new Error('ItemTracker is required');
}

const latestPrice = computed(() => model.value!.getLatestPrice())

const priceChange = computed(() => model.value!.getPriceChangeAsString())

const chartData = reactive({
    labels: model.value.getPriceData().dates.map(x => x.toLocaleDateString()),
    datasets: [
        {
            label: `Price ${model.value.getCurrency()}`,
            data: model.value.getPriceData().prices,
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
    // elements: {
    //     point: {
    //         radius: 0,
    //     },
    // },
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

const overlayPanelTrigger = ref();

const triggerOverlay = (e) => {
    overlayPanelTrigger.value.toggle(e);
}

const scope = getCurrentScope();
const app = getCurrentInstance();

const favorite = () => {
    scope?.run(() => {
        app?.appContext.app.runWithContext(() => {
            if (model.value!.isFavorite()) {
                unfavoriteApi(model.value!)
            } else {
                favoriteApi(model.value!)
            }
        })
    })
}
</script>

<template>
    <div class="col-12 lg:col-6 xl:col-3">
        <div
            class="card card-clickable"
            @click="triggerOverlay"
            >
            <h5>{{ model!.getName() }}</h5>
            <div class="card-details">
                <div class="card-divider" />
                <img
                    :src="model!.getIcon()"
                    height="100"
                    >
                <div class="card-price-details">
                    <h4 :class="`${priceChange[0] !== '-' ? 'text-green-500' : 'text-red-500'} text-center h-full mt-auto`">
                        {{ model!.getCurrency() }} {{ latestPrice }}
                    </h4>
                    <div class="card-price-divider" />
                    <h6 class="card-price-percentage">
                        ({{ priceChange || 0 }}%)
                    </h6>
                </div>
            </div>
        </div>
    </div>
    <OverlayPanel ref="overlayPanelTrigger">
        <div class="flex flex-column gap-1 w-20rem align-items-center">
            <Chart
                class="w-full chart-padding"
                type="line"
                :data="chartData"
                :options="chartOptions"
                />
            <div class="w-full flex gap-2 justify-between px-1">
                <a
                    :href="model!.getLink()"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="w-full h-3rem flex gap-2"
                    >
                    <Button
                        class="w-full h-full"
                        label="Visit"
                        />
                </a>
                <Button
                    class="w-2 h-3rem bg-pink-100 border-0 hover:bg-pink-200"
                    :icon="`pi ${model!.isFavorite() ? 'pi-heart-fill' : 'pi-heart'} text-xl text-pink-500`"
                    aria-label="Favorite"
                    @click="favorite"
                    />
            </div>
        </div>
    </OverlayPanel>
</template>
