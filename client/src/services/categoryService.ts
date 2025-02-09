import Category from '@/model/Category';
import { QueryClient, useQuery } from '@tanstack/vue-query';
import { reactive, watch } from 'vue';
import router from '@/router';
import CategoryComponent from '@/views/pages/Category.vue';

export function getAllCategories() {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['categories'],
        queryFn: fetchCategories,
    });

    watch(data, () => {
        const newCategories = data.value?.map((category) => ({
            path: `/category/${category.getId()}`,
            name: category.getName(),
            component: CategoryComponent,
            props: { category },
        }));

        if (newCategories)
            newCategories.forEach((c) => router.addRoute('main', c));
    });

    return reactive({ isPending, isError, data, error });
}

export async function fetchAllCategories(queryClient: QueryClient) {
    return queryClient.fetchQuery({
        queryKey: ['categories'],
        queryFn: fetchCategories,
    });
}

async function fetchCategories(): Promise<Category[]> {
    return fetch(new URL('/category', import.meta.env.VITE_API_BASE_URL), {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) =>
            data.Data.map(
                (category: { id: number; name: string }) =>
                    new Category(category.id, category.name),
            ),
        );
}

export async function addCategory(
    categoryName: string,
    categoryUrl: string,
): Promise<number> {
    const response = await fetch(
        new URL('/category', import.meta.env.VITE_API_BASE_URL),
        {
            method: 'POST',
            body: JSON.stringify({ name: categoryName, url: categoryUrl }),
        },
    );
    return response.status;
}

export async function getCategoryConfig(categoryId: number): Promise<string> {
    return await fetch(
        new URL(`/category/${categoryId}`, import.meta.env.VITE_API_BASE_URL),
        {
            method: 'GET',
        },
    )
        .then((res) => res.json())
        .then((data) => data.config);
}

export async function updateCategoryConfig(
    categoryId: number,
    config: string,
): Promise<number> {
    const response = await fetch(
        new URL(`/category/${categoryId}`, import.meta.env.VITE_API_BASE_URL),
        {
            method: 'PUT',
            body: JSON.stringify({ config: config }),
        },
    );
    return response.status;
}
