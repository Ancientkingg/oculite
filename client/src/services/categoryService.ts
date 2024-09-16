import Category from '@/model/Category';
import { QueryClient, useQuery } from '@tanstack/vue-query';
import { reactive } from 'vue';

export function getAllCategories() {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['categories'],
        queryFn: fetchCategories,
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
    return [new Category(0, 'blabla')];

    return fetch(import.meta.env.VITE_API_BASE_URL + 'category', {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) => data.Data);
}

export async function addCategory(
    categoryName: string,
    categoryUrl: string,
): Promise<number> {
    const response = await fetch(
        import.meta.env.VITE_API_BASE_URL + 'category',
        {
            method: 'POST',
            body: JSON.stringify({ name: categoryName, url: categoryUrl }),
        },
    );
    return response.status;
}

export async function getCategoryConfig(categoryId: number): Promise<string> {
    return await fetch(
        import.meta.env.VITE_API_BASE_URL + `category/${categoryId}`,
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
        import.meta.env.VITE_API_BASE_URL + `category/${categoryId}`,
        {
            method: 'PUT',
            body: JSON.stringify({ config: config }),
        },
    );
    return response.status;
}
