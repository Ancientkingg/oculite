<script lang="ts" setup>
import Category from '@/model/Category';
import { getAllItemTrackerIds, getItemTracker } from '@/services/itemTrackerService';
import { computed, getCurrentInstance, getCurrentScope, reactive, watch } from 'vue';
import ItemTrackerModel from '@/model/ItemTracker'

const props = defineProps<{ category: Category }>();

if (!props.category) {
    throw new Error('Category not found');
}

const fetchedItemTrackers = getAllItemTrackerIds(props.category);

const isLoading = computed(() => fetchedItemTrackers.isPending || fetchedItemTrackers.isError);

const itemTrackerResponses: { data: {
    isPending: boolean;
    isError: boolean;
    data: ItemTrackerModel | undefined;
    error: Error | null;
}[] } = reactive({ data: [] });

const scope = getCurrentScope();
const app = getCurrentInstance();

watch(fetchedItemTrackers, () => {
    scope?.run(() => {
        app?.appContext.app.runWithContext(() => {
            itemTrackerResponses.data = isLoading.value ? [] : fetchedItemTrackers.data!.map((id) => getItemTracker(id))
        })
    })
}, { immediate: true });
</script>

<template>
    <div
        class="grid p-fluid"
        style="margin-bottom: -1rem;"
        >
        <template
            v-if="isLoading"
            >
            <div
                v-for="n in 12"

                :key="n"
                class="col-12 lg:col-6 xl:col-3"
                >
                <Skeleton
                    height="13.5rem"
                    width="100%"
                    borderRadius="12px"
                    />
            </div>
        </template>

        <template
            v-for="(itemTracker, i) in itemTrackerResponses.data"
            :key="i"
            >
            <ItemTracker
                v-if="!itemTracker.isError && !itemTracker.isPending"
                v-model="itemTrackerResponses.data[i].data"
                />
            <div
                v-else
                class="col-12 lg:col-6 xl:col-3"
                >
                <Skeleton
                    height="13.5rem"
                    width="100%"
                    borderRadius="12px"
                    />
            </div>
        </template>
    </div>
</template>
