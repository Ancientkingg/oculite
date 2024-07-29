<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { useLayout } from '@/layout/composables/layout';

const { isDarkTheme } = useLayout();

const items: Ref = ref(null);
const lineOptions: Ref = ref(null);

const formatCurrency = (value: { toLocaleString: (arg0: string, arg1: { style: string; currency: string; }) => unknown; }) => {
    return value.toLocaleString('en-US', { style: 'currency', currency: 'USD' });
};
const applyLightTheme = () => {
    lineOptions.value = {
        plugins: {
            legend: {
                labels: {
                    color: '#495057'
                }
            }
        },
        scales: {
            x: {
                ticks: {
                    color: '#495057'
                },
                grid: {
                    color: '#ebedef'
                }
            },
            y: {
                ticks: {
                    color: '#495057'
                },
                grid: {
                    color: '#ebedef'
                }
            }
        }
    };
};

const applyDarkTheme = () => {
    lineOptions.value = {
        plugins: {
            legend: {
                labels: {
                    color: '#ebedef'
                }
            }
        },
        scales: {
            x: {
                ticks: {
                    color: '#ebedef'
                },
                grid: {
                    color: 'rgba(160, 167, 181, .3)'
                }
            },
            y: {
                ticks: {
                    color: '#ebedef'
                },
                grid: {
                    color: 'rgba(160, 167, 181, .3)'
                }
            }
        }
    };
};

watch(
    isDarkTheme,
    (val) => {
        if (val) {
            applyDarkTheme();
        } else {
            applyLightTheme();
        }
    },
    { immediate: true }
);
</script>

<template>
    <div class="grid">
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Watching</span>
                        <div class="text-900 font-medium text-xl">
                            152
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-blue-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-eye text-blue-500 text-xl" />
                    </div>
                </div>
                <span class="text-green-500 font-medium">24 new </span>
                <span class="text-500">since last visit</span>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Rising Trackers</span>
                        <div class="text-900 font-medium text-xl">
                            152
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-red-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-caret-up text-red-500 text-xl" />
                    </div>
                </div>
                <span class="text-green-500 font-medium">24 new </span>
                <span class="text-500">since last visit</span>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Falling Trackers</span>
                        <div class="text-900 font-medium text-xl">
                            152
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-green-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-caret-down text-green-500 text-xl" />
                    </div>
                </div>
                <span class="text-green-500 font-medium">24 new </span>
                <span class="text-500">since last visit</span>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Stale Trackers</span>
                        <div class="text-900 font-medium text-xl">
                            28441
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-gray-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-minus text-gray-500 text-xl" />
                    </div>
                </div>
                <span class="text-green-500 font-medium">520 </span>
                <span class="text-500">newly registered</span>
            </div>
        </div>
        <div class="col-12 xl:col-6">
            <div class="card">
                <div class="flex justify-content-between mb-3">
                    <h5>Favorited Trackers</h5>
                    <div
                        class="flex align-items-center justify-content-center bg-pink-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-heart text-pink-500 text-xl" />
                    </div>
                </div>
                <DataTable
                    :value="items"
                    :rows="5"
                    :paginator="true"
                    responsive-layout="scroll"
                    >
                    <Column style="width: 15%">
                        <template #header>
                            Image
                        </template>
                    </Column>
                    <Column
                        field="name"
                        header="Name"
                        :sortable="true"
                        style="width: 35%"
                        />
                    <Column
                        field="price"
                        header="Price"
                        :sortable="true"
                        style="width: 35%"
                        >
                        <template #body="slotProps">
                            {{ formatCurrency(slotProps.data.price) }}
                        </template>
                    </Column>
                    <Column style="width: 15%">
                        <template #header>
                            View
                        </template>
                        <template #body>
                            <Button
                                icon="pi pi-search"
                                type="button"
                                class="p-button-text"
                                />
                        </template>
                    </Column>
                </DataTable>
            </div>
        </div>
        <div class="col-12 xl:col-6">
            <div class="card">
                <div class="flex align-items-center justify-content-between mb-4">
                    <h5>Notifications</h5>
                    <div
                        class="flex align-items-center justify-content-center bg-purple-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-send text-purple-500 text-xl" />
                    </div>
                </div>

                <span class="block text-600 font-medium mb-3">TODAY</span>
                <ul class="p-0 mx-0 mt-0 mb-4 list-none">
                    <li class="flex align-items-center py-2 border-bottom-1 surface-border">
                        <div class="w-3rem h-3rem flex align-items-center justify-content-center bg-blue-100 border-circle mr-3 flex-shrink-0">
                            <i class="pi pi-dollar text-xl text-blue-500" />
                        </div>
                        <span class="text-900 line-height-3">Richard Jones
                            <span class="text-700">has purchased a blue t-shirt for <span class="text-blue-500">79$</span></span>
                        </span>
                    </li>
                    <li class="flex align-items-center py-2">
                        <div class="w-3rem h-3rem flex align-items-center justify-content-center bg-orange-100 border-circle mr-3 flex-shrink-0">
                            <i class="pi pi-download text-xl text-orange-500" />
                        </div>
                        <span class="text-700 line-height-3">Your request for withdrawal of <span class="text-blue-500 font-medium">2500$</span> has been initiated.</span>
                    </li>
                </ul>

                <span class="block text-600 font-medium mb-3">YESTERDAY</span>
                <ul class="p-0 m-0 list-none">
                    <li class="flex align-items-center py-2 border-bottom-1 surface-border">
                        <div class="w-3rem h-3rem flex align-items-center justify-content-center bg-blue-100 border-circle mr-3 flex-shrink-0">
                            <i class="pi pi-dollar text-xl text-blue-500" />
                        </div>
                        <span class="text-900 line-height-3">Keyser Wick
                            <span class="text-700">has purchased a black jacket for <span class="text-blue-500">59$</span></span>
                        </span>
                    </li>
                    <li class="flex align-items-center py-2 border-bottom-1 surface-border">
                        <div class="w-3rem h-3rem flex align-items-center justify-content-center bg-pink-100 border-circle mr-3 flex-shrink-0">
                            <i class="pi pi-question text-xl text-pink-500" />
                        </div>
                        <span class="text-900 line-height-3">Jane Davis
                            <span class="text-700">has posted a new questions about your product.</span>
                        </span>
                    </li>
                </ul>
            </div>
        </div>
    </div>
</template>
