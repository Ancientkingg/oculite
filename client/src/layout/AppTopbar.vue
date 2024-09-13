<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, Ref } from 'vue';
import { useLayout } from '@/layout/composables/layout';
import { showToast } from '@/layout/composables/toast';
import { useToast } from 'primevue/usetoast';
import categoryService from '@/services/categoryService';

const { layoutConfig, onMenuToggle } = useLayout();

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
const display = ref(false);

const open = () => {
    display.value = true;
};

const toast = useToast();

const cancel = () => {
    display.value = false;
    showToast(toast, 'error', 'Cancelled', 'Category addition cancelled', 1000);
};

const confirm = async () => {
    if (categoryName.value === '' || categoryUrl.value === '') return;

    const responseStatus = await categoryService.addCategory(categoryName.value, categoryUrl.value);

    display.value = false;

    if (responseStatus === 200) {
        showToast(toast, 'success', 'Success', 'Category added successfully', 1000);
    } else {
        showToast(toast, 'error', 'Error', 'Could not add category', 1000);
    }
};

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
                class="p-link layout-topbar-button"
                @click="open"
                >
                <i class="pi pi-plus" />
                <span>Add Category</span>
            </button>
            <Dialog
                v-model:visible="display"
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
                        @click="cancel"
                        />
                    <Button
                        label="Confirm"
                        icon="pi pi-check"
                        class="p-button-outlined"
                        @click="confirm"
                        />
                </template>
            </Dialog>
        </div>
    </div>
</template>

<style lang="scss" scoped></style>
