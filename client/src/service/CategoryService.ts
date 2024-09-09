export interface Category {
    id: number;
    name: string;
}

class CategoryService {
    async getCategories(): Promise<Category[]> {
        return new Promise((resolve) => {
            setTimeout(() => resolve([{ id: 0, name: 'blabla' }]), 1000);
        });
    }
}

export default new CategoryService();
