<script setup>
import { ref, computed } from 'vue';

import AppMenuItem from './AppMenuItem.vue';
import categoryService from '@/service/categoryService';

const categories = ref([]);
categoryService.getCategories().then(c => {
    categories.value = c;
});

const categoryItems = computed(() =>
    categories.value.map(category => ({
        label: category.name,
        icon: 'pi pi-fw pi-list',
        to: `/category/${category.id}`
    })))

const model = computed(() => [
    {
        label: 'Home',
        items: [{ label: 'Dashboard', icon: 'pi pi-fw pi-home', to: '/' }]
    },
    {
        label: 'Categories',
        items: categoryItems.value
    }
]);
</script>

<template>
    <ul class="layout-menu">
        <template
            v-for="(item, i) in model"
            :key="item"
            >
            <app-menu-item
                v-if="!item.separator"
                :item="item"
                :index="i"
                />
            <li
                v-if="item.separator"
                class="menu-separator"
                />
        </template>
    </ul>
</template>

<style lang="scss" scoped></style>
