<script lang="ts" setup>
    import { Notification } from '@/services/statsService';
import { computed } from 'vue';

    interface Props {
        notifications: Notification[];
    }


    const props = defineProps<Props>();

    const groupBy = <T, >(list: T[], keyGetter: (item: T) => any) => {
        const map = new Map();
        list.forEach((item) => {
             const key = keyGetter(item);
             const collection = map.get(key);
             if (!collection) {
                 map.set(key, [item]);
             } else {
                 collection.push(item);
             }
        });
        return map;
    }

    const dateDiffInDays = (a: Date, b: Date): number => {
      const _MS_PER_DAY = 1000 * 60 * 60 * 24;
      // Discard the time and time-zone information.
      const utc1 = Date.UTC(a.getFullYear(), a.getMonth(), a.getDate());
      const utc2 = Date.UTC(b.getFullYear(), b.getMonth(), b.getDate());

      return Math.floor((utc2 - utc1) / _MS_PER_DAY);
    }

    const groupNotificationsByDate = (notifications: Notification[]) => {
        return groupBy(notifications, notification => dateDiffInDays(new Date(), notification.date));
    }

    const getDayName = (daysFromToday: number): string => {
        if (daysFromToday === 0) return 'TODAY';
        if (daysFromToday === -1) return 'YESTERDAY';

        const date = new Date();
        date.setDate(date.getDate() - daysFromToday);

        return date.toLocaleDateString();
    }

    const groupedNotifications = computed(() => groupNotificationsByDate([...props.notifications].sort((a: any,b: any) => b.date - a.date)));
</script>


<template>
    <div
        v-for="[key, value] in groupedNotifications.entries()"
        :key="key"
        >
        <span
            class="block text-600 font-medium mb-3"
            > {{ getDayName(key) }}</span>
        <ul class="p-0 mx-0 mt-0 mb-4 list-none">
            <NotificationItem
                v-for="notification in value"
                :key="notification.date"
                :notification="notification"
                />
        </ul>
    </div>
</template>

