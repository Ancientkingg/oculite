<script setup lang="ts">
import { Ref, ref, watch, reactive, computed, getCurrentScope, getCurrentInstance } from 'vue';
import { useLayout } from '@/layout/composables/layout';

const { isDarkTheme } = useLayout();
import { getItemTracker } from '@/services/itemTrackerService';
import ItemTracker from '@/model/ItemTracker';
import { getFavoriteTrackers, getNotifications, getTrackerStats } from '@/services/statsService';


const trackerStats = getTrackerStats();
const favoritedTrackers = getFavoriteTrackers();

const itemTrackerResponses: { data: {
    isPending: boolean;
    isError: boolean;
    data: ItemTracker | undefined;
    error: Error | null;
}[] } = reactive({ data: [] });

const favoritedIsLoading = computed(() => favoritedTrackers.isPending || favoritedTrackers.isError);

const notificationsResponse = getNotifications();

const scope = getCurrentScope();
const app = getCurrentInstance();

watch(favoritedTrackers, () => {
    scope?.run(() => {
        app?.appContext.app.runWithContext(() => {
            itemTrackerResponses.data = favoritedIsLoading.value ? [] : favoritedTrackers.data!.map((id) => getItemTracker(id))
        })
    })
}, { immediate: true })

const lineOptions: Ref = ref(null);

const formatCurrency = (value: { toLocaleString: (arg0: string, arg1: { style: string; currency: string; }) => unknown; }) => {
    return value.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });
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
                        <ProgressSpinner
                            v-if="!trackerStats.data"
                            style="width: 16px; height: 16px"
                            strokeWidth="8"
                            animationDuration=".5s"
                            />
                        <div
                            v-else
                            class="text-900 font-medium text-xl"
                            >
                            {{ trackerStats.data.total }}
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-blue-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-eye text-blue-500 text-xl" />
                    </div>
                </div>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Rising Trackers</span>
                        <ProgressSpinner
                            v-if="!trackerStats.data"
                            style="width: 16px; height: 16px"
                            strokeWidth="8"
                            animationDuration=".5s"
                            />
                        <div
                            v-else
                            class="text-900 font-medium text-xl"
                            >
                            {{ trackerStats.data.rising }}
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-red-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-caret-up text-red-500 text-xl" />
                    </div>
                </div>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Falling Trackers</span>
                        <ProgressSpinner
                            v-if="!trackerStats.data"
                            style="width: 16px; height: 16px"
                            strokeWidth="8"
                            animationDuration=".5s"
                            />
                        <div
                            v-else
                            class="text-900 font-medium text-xl"
                            >
                            {{ trackerStats.data.falling }}
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-green-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-caret-down text-green-500 text-xl" />
                    </div>
                </div>
            </div>
        </div>
        <div class="col-12 lg:col-6 xl:col-3">
            <div class="card mb-0">
                <div class="flex justify-content-between mb-3">
                    <div>
                        <span class="block text-500 font-medium mb-3">Stale Trackers</span>
                        <ProgressSpinner
                            v-if="!trackerStats.data"
                            style="width: 16px; height: 16px"
                            strokeWidth="8"
                            animationDuration=".5s"
                            />
                        <div
                            v-else
                            class="text-900 font-medium text-xl"
                            >
                            {{ trackerStats.data.stale }}
                        </div>
                    </div>
                    <div
                        class="flex align-items-center justify-content-center bg-gray-100 border-round"
                        style="width: 2.5rem; height: 2.5rem"
                        >
                        <i class="pi pi-minus text-gray-500 text-xl" />
                    </div>
                </div>
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
                    :value="itemTrackerResponses.data"
                    :rows="5"
                    :paginator="true"
                    responsive-layout="scroll"
                    :loading="favoritedIsLoading"
                    >
                    <Column style="width: 15%">
                        <template #header>
                            Image
                        </template>
                        <template #body="slotProps">
                            <ProgressSpinner
                                v-if="!slotProps.data.data"
                                style="width: 16px; height: 16px"
                                strokeWidth="8"
                                animationDuration=".5s"
                                />
                            <img
                                v-else
                                :src="slotProps.data.data.icon"
                                class="w-5rem border-round"
                                >
                        </template>
                    </Column>
                    <Column
                        header="Name"
                        :sortable="true"
                        style="width: 35%"
                        >
                        <template #body="slotProps">
                            <ProgressSpinner
                                v-if="!slotProps.data.data"
                                style="width: 16px; height: 16px"
                                strokeWidth="8"
                                animationDuration=".5s"
                                />
                            <div v-else>
                                {{ !slotProps.data.data ? '' : slotProps.data.data.getName() }}
                            </div>
                        </template>
                    </Column>
                    <Column
                        header="Price"
                        :sortable="true"
                        style="width: 35%"
                        >
                        <template #body="slotProps">
                            <ProgressSpinner
                                v-if="!slotProps.data.data"
                                style="width: 16px; height: 16px"
                                strokeWidth="8"
                                animationDuration=".5s"
                                />
                            <div v-else>
                                {{ !slotProps.data.data ? 0 : formatCurrency(slotProps.data.data.getLatestPrice()) }}
                            </div>
                        </template>
                    </Column>
                    <Column
                        field="url"
                        style="width: 15%"
                        >
                        <template #header>
                            View
                        </template>
                        <template #body="slotProps">
                            <ProgressSpinner
                                v-if="!slotProps.data.data"
                                style="width: 16px; height: 16px"
                                strokeWidth="8"
                                animationDuration=".5s"
                                />
                            <a
                                v-else
                                :href="!slotProps.data.data ? '' : slotProps.data.data.link"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="w-full h-3rem flex gap-2"
                                >
                                <Button
                                    icon="pi pi-search"
                                    type="button"
                                    class="p-button-text w-8"
                                    />
                            </a>
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
                <ProgressSpinner
                    v-if="!notificationsResponse.data"
                    style="width: 48px; height: 25rem"
                    strokeWidth="8"
                    animationDuration=".5s"
                    />
                <div
                    v-else
                    style="max-height: 25rem; overflow-y: auto;"
                    >
                    <NotificationBlock :notifications="notificationsResponse.data" />
                </div>
            </div>
        </div>
    </div>
</template>
