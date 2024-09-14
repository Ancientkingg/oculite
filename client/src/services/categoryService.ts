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
            body: JSON.stringify({ categoryName, categoryUrl }),
        },
    );
    return response.status;
}
