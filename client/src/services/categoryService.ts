import Category from '@/model/Category';
import { QueryClient, useQuery } from '@tanstack/vue-query';
import { reactive } from 'vue';

class CategoryService {
    getAllCategories() {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['categories'],
            queryFn: this.fetchCategories,
        });

        return reactive({ isPending, isError, data, error });
    }

    async fetchAllCategories(queryClient: QueryClient) {
        return queryClient.fetchQuery({
            queryKey: ['categories'],
            queryFn: this.fetchCategories,
        });
    }

    private async fetchCategories(): Promise<Category[]> {
        return fetch(import.meta.env.VITE_API_BASE_URL + 'fetchCategory', {
            method: 'GET',
        })
            .then((res) => res.json())
            .then((data) => data.data);
    }

    async addCategory(
        categoryName: string,
        categoryUrl: string,
    ): Promise<number> {
        const response = await fetch(
            import.meta.env.VITE_API_BASE_URL + 'addCategory',
            {
                method: 'POST',
                body: JSON.stringify({ categoryName, categoryUrl }),
            },
        );
        return response.status;
    }
}

export default new CategoryService();
