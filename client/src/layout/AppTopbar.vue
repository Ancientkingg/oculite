<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useLayout } from '@/layout/composables/layout';
import { useRouter } from 'vue-router';

const { layoutConfig, onMenuToggle } = useLayout();

const outsideClickListener = ref(null);
const topbarMenuActive = ref(false);
const _router = useRouter();

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

const display = ref(false);

const open = () => {
    display.value = true;
};

const cancel = () => {
    display.value = false;
};

const confirm = () => {
    if (categoryName.value === '') return;

    // TODO add category using category service

    display.value = false;
};

const bindOutsideClickListener = () => {
    if (!outsideClickListener.value) {
        outsideClickListener.value = (event) => {
            if (isOutsideClicked(event)) {
                topbarMenuActive.value = false;
            }
        };
        document.addEventListener('click', outsideClickListener.value);
    }
};
const unbindOutsideClickListener = () => {
    if (outsideClickListener.value) {
        document.removeEventListener('click', outsideClickListener);
        outsideClickListener.value = null;
    }
};
const isOutsideClicked = (event) => {
    if (!topbarMenuActive.value) return;

    const sidebarEl = document.querySelector('.layout-topbar-menu');
    const topbarEl = document.querySelector('.layout-topbar-menu-button');

    return !(sidebarEl.isSameNode(event.target) || sidebarEl.contains(event.target) || topbarEl.isSameNode(event.target) || topbarEl.contains(event.target));
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
