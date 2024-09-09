import Category from '@/model/Category';
import ItemTracker from '@/model/ItemTracker';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';

const queryClient = useQueryClient();

class ItemTrackerService {
    favoriteMutation: any;
    unFavoriteMutation: any;

    constructor() {
        this.favoriteMutation = useMutation({
            mutationFn: (itemTrackerId: number) =>
                fetch('favorite', {
                    method: 'PUT',
                    body: JSON.stringify({ itemTrackerId }),
                }).then((res) => res.json()),

            onSuccess: (itemTrackerId: number) => {
                queryClient.invalidateQueries({
                    queryKey: ['item-tracker', itemTrackerId],
                });
            },
        });

        this.unFavoriteMutation = useMutation({
            mutationFn: (itemTrackerId: number) =>
                fetch('unfavorite', {
                    method: 'PUT',
                    body: JSON.stringify({ itemTrackerId }),
                }).then((res) => res.json()),
            onSuccess: (itemTrackerId: number) => {
                queryClient.invalidateQueries({
                    queryKey: ['item-tracker', itemTrackerId],
                });
            },
        });
    }

    getAllItemTrackers(category: Category) {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['categories', category.getId()],
            queryFn: () => this.fetchItemTrackers(category.getId()),
        });

        return { isPending, isError, data, error };
    }

    private async fetchItemTrackers(
        categoryId: number,
    ): Promise<ItemTracker[]> {
        return fetch(`item-trackers/${categoryId}`, { method: 'GET' })
            .then((res) => res.json())
            .then((data) => data.data);
    }

    favorite(item: ItemTracker) {
        return this.favoriteMutation.mutate(item.getId());
    }

    unfavorite(item: ItemTracker) {
        return this.unFavoriteMutation.mutate(item.getId());
    }
}

export default new ItemTrackerService();
