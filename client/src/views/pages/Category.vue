<script lang="ts" setup>
import Category from '@/model/Category';
import { getAllItemTrackerIds, getItemTracker } from '@/services/itemTrackerService';
import { computed, getCurrentInstance, getCurrentScope, reactive, watch } from 'vue';

const props = defineProps<{ category: Category }>();

if (!props.category) {
    throw new Error('Category not found');
}

const fetchedItemTrackers = getAllItemTrackerIds(props.category);

const isLoading = computed(() => fetchedItemTrackers.isPending || fetchedItemTrackers.isError);

const itemTrackerResponses: { data: any[] } = reactive({ data: [] })

const scope = getCurrentScope();
const app = getCurrentInstance();

watch(fetchedItemTrackers, () => {
    scope?.run(() => {
        app?.appContext.app.runWithContext(() => {
            itemTrackerResponses.data = isLoading.value ? [] : fetchedItemTrackers.data!.map((id) => getItemTracker(id))
        })
    })
}, { immediate: true })

</script>

<template>
    <div class="grid p-fluid">
        <div
            v-if="isLoading"
            class="w-full h-full m-0 p-0 grid"
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
        </div>

        <div
            v-for="(itemTracker, i) in itemTrackerResponses.data"
            :key="i"
            class="w-full h-full m-0 p-0 grid"
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
        </div>
    </div>
</template>
