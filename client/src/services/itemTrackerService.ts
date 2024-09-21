import Category from '@/model/Category';
import ItemTracker from '@/model/ItemTracker';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { reactive } from 'vue';

export function getAllItemTrackerIds(category: Category) {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['item-trackers', category.getId()],
        queryFn: () => fetchAllItemTrackerIds(category.getId()),
    });

    return reactive({ isPending, isError, data, error });
}

export function getItemTracker(id: number) {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['item-tracker', id],
        queryFn: () => fetchItemTracker(id),
    });

    return reactive({ isPending, isError, data, error });
}

async function fetchItemTracker(id: number): Promise<ItemTracker> {
    return fetch(import.meta.env.VITE_API_BASE_URL + `it/${id}`, {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) => {
            return new ItemTracker(
                data.id,
                data.name,
                data.priceData,
                data.currency,
                data.icon,
                data.link,
                data.favorite,
            );
        });
}

async function fetchAllItemTrackerIds(categoryId: number): Promise<number[]> {
    return fetch(
        import.meta.env.VITE_API_BASE_URL + `category/${categoryId}/it`,
        {
            method: 'GET',
        },
    )
        .then((res) => res.json())
        .then((data) => data.Data);
}

export function favorite(item: ItemTracker) {
    const queryClient = useQueryClient();
    const favoriteMutation = useMutation({
        mutationFn: (itemTrackerId: number) =>
            fetch(
                import.meta.env.VITE_API_BASE_URL +
                    `it/${itemTrackerId}/` +
                    'favorite',
                {
                    method: 'PUT',
                },
            ).then((res) => res.json()),

        onSuccess: (itemTrackerId: number) => {
            queryClient.invalidateQueries({
                queryKey: ['item-tracker', itemTrackerId],
            });
        },
    });

    return favoriteMutation.mutate(item.getId());
}

export function unfavorite(item: ItemTracker) {
    const queryClient = useQueryClient();
    const unFavoriteMutation = useMutation({
        mutationFn: (itemTrackerId: number) =>
            fetch(
                import.meta.env.VITE_API_BASE_URL +
                    `it/${itemTrackerId}/` +
                    'unfavorite',
                {
                    method: 'PUT',
                },
            ).then((res) => res.json()),
        onSuccess: (itemTrackerId: number) => {
            queryClient.invalidateQueries({
                queryKey: ['item-tracker', itemTrackerId],
            });
        },
    });
    return unFavoriteMutation.mutate(item.getId());
}
