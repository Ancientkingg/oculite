import Category from '@/model/Category';
import { useQuery } from '@tanstack/vue-query';

class CategoryService {
    getAllCategories() {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['categories'],
            queryFn: this.fetchCategories,
        });

        return { isPending, isError, data, error };
    }

    private async fetchCategories(): Promise<Category[]> {
        return new Promise((resolve) => {
            setTimeout(() => resolve([new Category(0, 'blabla')]), 1000);
        });
    }
}

export default new CategoryService();
