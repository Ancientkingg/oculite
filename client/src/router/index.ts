import { createRouter, createWebHistory } from 'vue-router';
import AppLayout from '@/layout/AppLayout.vue';
import { QueryClient } from '@tanstack/vue-query';
import { fetchAllCategories } from '@/services/categoryService';

const queryClient = new QueryClient();

const categories = await (async () => {
    try {
        const categories = await fetchAllCategories(queryClient);
        return categories;
    } catch (e) {
        return [];
    }
})();

const categoryRoutes = categories.map((category) => ({
    path: `/category/${category.getId()}`,
    name: category.getName(),
    component: () => import('@/views/pages/Category.vue'),
    props: { category },
}));

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: AppLayout,
            children: [
                {
                    path: '/',
                    name: 'dashboard',
                    component: () => import('@/views/Dashboard.vue'),
                },
                ...categoryRoutes,
            ],
        },
        {
            path: '/:pathMatch(.*)*',
            name: 'notfound',
            component: () => import('@/views/pages/NotFound.vue'),
        },
    ],
});

export default router;
