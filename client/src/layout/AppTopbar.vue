<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, Ref, getCurrentScope, getCurrentInstance } from 'vue';
import { useLayout } from '@/layout/composables/layout';
import { showToast } from '@/layout/composables/toast';
import { useToast } from 'primevue/usetoast';
import { addCategory, getCategoryConfig, updateCategoryConfig } from '@/services/categoryService';
import { useQueryClient } from '@tanstack/vue-query';
const { layoutConfig, onMenuToggle } = useLayout();

interface Props {
    showCategoryConfig: boolean;
    categoryId: number;
}

const props = defineProps<Props>();

const outsideClickListener: Ref = ref(null);
const topbarMenuActive: Ref<boolean> = ref(false);

onMounted(() => {
    bindOutsideClickListener();
});

onBeforeUnmount(() => {
    unbindOutsideClickListener();
});

const logoUrl = computed(() => {
    return `/layout/images/${layoutConfig.darkTheme.value ? 'logo-white' : 'logo-dark'}.png`;
});

const onTopBarMenuButton = () => {
    topbarMenuActive.value = !topbarMenuActive.value;
};

const topbarMenuClasses = computed(() => {
    return {
        'layout-topbar-menu-mobile-active': topbarMenuActive.value
    };
});

const categoryName = ref('');
const categoryUrl = ref('');

const categoryConfig = ref('');

if (props.showCategoryConfig) {
    getCategoryConfig(props.categoryId).catch(() => '').then((x) => {
        categoryConfig.value = x;
    })
}

const displayAddCategory = ref(false);
const displayCategoryConfig = ref(false);

const openAddCategory = () => {
    displayAddCategory.value = true;
};

const openCategoryConfig = () => {
    displayCategoryConfig.value = true;
}

const toast = useToast();

const cancelAddCategory = () => {
    displayAddCategory.value = false;
    showToast(toast, 'error', 'Cancelled', 'Category addition cancelled', 1000);
};

const cancelCategoryConfig = () => {
    displayCategoryConfig.value = false;
    showToast(toast, 'error', 'Cancelled', 'Category Config edit cancelled', 1000);
};

const scope = getCurrentScope();
const app = getCurrentInstance();

const confirmAddCategory = async () => {
    if (categoryName.value === '' || categoryUrl.value === '') return;

    const responseStatus = await addCategory(categoryName.value, categoryUrl.value);

    displayAddCategory.value = false;

    if (responseStatus >= 200 && responseStatus < 300) {
        scope?.run(() => {
            app?.appContext.app.runWithContext(() => {
                const queryClient = useQueryClient();
                queryClient.invalidateQueries({
                    queryKey: ['categories'],
                });
            })
        })

        categoryName.value = '';
        categoryUrl.value = '';

        showToast(toast, 'success', 'Success', 'Category added successfully', 1000);
    } else {
        showToast(toast, 'error', 'Error', 'Could not add category', 1000);
    }
};

const confirmCategoryConfig = async () => {
    const responseStatus = await updateCategoryConfig(props.categoryId, categoryConfig.value);

    displayCategoryConfig.value = false;

    if (responseStatus >= 200 && responseStatus < 300) {
        showToast(toast, 'success', 'Success', 'Category config updated successfully', 1000);
    } else {
        showToast(toast, 'error', 'Error', 'Could not update category config', 1000);
    }
}

const bindOutsideClickListener = () => {
    if (!outsideClickListener.value) {
        outsideClickListener.value = (event: any) => {
            if (isOutsideClicked(event)) {
                topbarMenuActive.value = false;
            }
        };
        document.addEventListener('click', outsideClickListener.value);
    }
};
const unbindOutsideClickListener = () => {
    if (outsideClickListener.value) {
        document.removeEventListener('click', outsideClickListener.value);
        outsideClickListener.value = null;
    }
};
const isOutsideClicked = (event) => {
    if (!topbarMenuActive.value) return;

    const sidebarEl = document.querySelector('.layout-topbar-menu');
    const topbarEl = document.querySelector('.layout-topbar-menu-button');

    return !(sidebarEl!.isSameNode(event.target) || sidebarEl!.contains(event.target) || topbarEl!.isSameNode(event.target) || topbarEl!.contains(event.target));
};
</script>

<template>
    <div class="layout-topbar">
        <router-link
            to="/"
            class="layout-topbar-logo"
            >
            <img
                :src="logoUrl"
                alt="logo"
                >
            <span>Oculite</span>
        </router-link>

        <button
            class="p-link layout-menu-button layout-topbar-button"
            @click="onMenuToggle()"
            >
            <i class="pi pi-bars" />
        </button>

        <button
            class="p-link layout-topbar-menu-button layout-topbar-button"
            @click="onTopBarMenuButton()"
            >
            <i class="pi pi-ellipsis-v" />
        </button>

        <div
            class="layout-topbar-menu"
            :class="topbarMenuClasses"
            >
            <button
                v-if="props.showCategoryConfig"
                class="p-link layout-topbar-button"
                @click="openCategoryConfig"
                >
                <i class="pi pi-file-edit" />
                <span>Edit Category Config</span>
            </button>
            <button
                class="p-link layout-topbar-button"
                @click="openAddCategory"
                >
                <i class="pi pi-plus" />
                <span>Add Category</span>
            </button>
            <Dialog
                v-model:visible="displayAddCategory"
                header="Add Category"
                :breakpoints="{ '960px': '75vw' }"
                :style="{ width: '30vw' }"
                :modal="true"
                :dismissableMask="true"
                :draggable="false"
                >
                <div class="p-fluid field pt-4">
                    <FloatLabel>
                        <InputText
                            id="category-name"
                            v-model="categoryName"
                            size="large"
                            type="text"
                            :invalid="categoryName === ''"
                            />
                        <label for="category-name">Name</label>
                    </FloatLabel>
                </div>
                <div class="p-fluid field pt-3">
                    <FloatLabel>
                        <InputText
                            id="category-url"
                            v-model="categoryUrl"
                            size="large"
                            type="text"
                            :invalid="categoryUrl === ''"
                            />
                        <label for="category-url">Url</label>
                    </FloatLabel>
                </div>
                <template #footer>
                    <Button
                        label="Cancel"
                        icon="pi pi-times"
                        class="p-button-outlined"
                        @click="cancelAddCategory"
                        />
                    <Button
                        label="Confirm"
                        icon="pi pi-check"
                        class="p-button-outlined"
                        @click="confirmAddCategory"
                        />
                </template>
            </Dialog>
            <Dialog
                v-if="props.showCategoryConfig"
                v-model:visible="displayCategoryConfig"
                header="Edit Category Config"
                :breakpoints="{ '960px': '75vw' }"
                :style="{ width: '30vw' }"
                :modal="true"
                :dismissableMask="true"
                :draggable="false"
                >
                <div class="p-fluid field pt-4">
                    <FloatLabel>
                        <Textarea
                            id="category-config"
                            v-model="categoryConfig"
                            autoResize
                            rows="12"
                            cols="30"
                            />
                        <!-- <label for="category-config">Category Config</label> -->
                    </FloatLabel>
                </div>
                <template #footer>
                    <Button
                        label="Cancel"
                        icon="pi pi-times"
                        class="p-button-outlined"
                        @click="cancelCategoryConfig"
                        />
                    <Button
                        label="Confirm"
                        icon="pi pi-check"
                        class="p-button-outlined"
                        @click="confirmCategoryConfig"
                        />
                </template>
            </Dialog>
        </div>
    </div>
</template>

<style lang="scss" scoped></style>
