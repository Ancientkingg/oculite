<script lang="ts" setup>
import { computed, reactive } from 'vue';

import AppMenuItem from './AppMenuItem.vue';
import categoryService from '@/services/categoryService';
import Category from '@/model/Category';

const categories = reactive(categoryService.getAllCategories());

const categoryItems = computed(() => categories.isPending ? [] :
    categories.data!.map((category: Category) => ({
        label: category.getName(),
        icon: 'pi pi-fw pi-list',
        to: `/category/${category.getId()}`
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
                :item="item"
                :index="i"
                />
            <li
                class="menu-separator"
                />
        </template>
    </ul>
</template>

<style lang="scss" scoped></style>
