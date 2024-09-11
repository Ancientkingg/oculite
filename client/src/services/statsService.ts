import { useQuery } from '@tanstack/vue-query';

export interface TrackerStats {
    total: number;
    rising: number;
    falling: number;
    stale: number;
}

export interface Notification {
    message: string;
    icon: string;
}

class StatsService {
    async fetchHealth() {
        return fetch('health', { method: 'GET' });
    }

    async serverIsOk(): Promise<boolean> {
        return (await this.fetchHealth()).ok;
    }

    getTrackerStats() {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['total-trackers'],
            queryFn: () => this.fetchTrackerStats(),
        });

        return { isPending, isError, data, error };
    }

    private async fetchTrackerStats(): Promise<TrackerStats> {
        return fetch('trackers', { method: 'GET' })
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

    getNotifications() {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['notifications'],
            queryFn: () => this.fetchNotifications(),
        });

        return { isPending, isError, data, error };
    }

    private async fetchNotifications(): Promise<Notification[]> {
        return fetch('notifications', { method: 'GET' })
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

    getFavoriteTrackers() {
        const { isPending, isError, data, error } = useQuery({
            queryKey: ['favorite-trackers'],
            queryFn: () => this.fetchFavoriteTrackers(),
        });

        return { isPending, isError, data, error };
    }

    private async fetchFavoriteTrackers(): Promise<number[]> {
        return fetch('favorite-trackers', { method: 'GET' })
            .then((res) => res.json())
            .then((data) => data.data);
    }
}

export default new StatsService();
