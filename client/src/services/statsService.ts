import { useQuery } from '@tanstack/vue-query';
import { reactive } from 'vue';
import Notification from '@/model/Notification';
import TrackerStats from '@/model/TrackerStats';

export async function fetchHealth() {
    return fetch(import.meta.env.VITE_API_BASE_URL + 'health', {
        method: 'GET',
    });
}

export async function serverIsOk(): Promise<boolean> {
    return (await fetchHealth()).ok;
}

export function getTrackerStats() {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['total-trackers'],
        queryFn: () => fetchTrackerStats(),
    });

    return reactive({ isPending, isError, data, error });
}

async function fetchTrackerStats(): Promise<TrackerStats> {
    return fetch(import.meta.env.VITE_API_BASE_URL + 'trackers', {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) => {
            return {
                total: data.total,
                rising: data.rising,
                falling: data.falling,
                stale: data.stale,
            };
        });
}

export function getNotifications() {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['notifications'],
        queryFn: () => fetchNotifications(),
    });

    return reactive({ isPending, isError, data, error });
}

async function fetchNotifications(): Promise<Notification[]> {
    return fetch(import.meta.env.VITE_API_BASE_URL + 'notifications', {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) => {
            return data.data.map((notification: any) => {
                return {
                    message: notification.message,
                    icon: notification.icon,
                };
            });
        });
}

export function getFavoriteTrackers() {
    const { isPending, isError, data, error } = useQuery({
        queryKey: ['favorite-trackers'],
        queryFn: () => fetchFavoriteTrackers(),
    });

    return reactive({ isPending, isError, data, error });
}

async function fetchFavoriteTrackers(): Promise<number[]> {
    return fetch(import.meta.env.VITE_API_BASE_URL + 'favorite-trackers', {
        method: 'GET',
    })
        .then((res) => res.json())
        .then((data) => data.data);
}
