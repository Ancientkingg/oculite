<script lang="ts" setup>
    import Notification from '@/model/Notification';

    interface Props {
        notification: Notification
    }

    const props = defineProps<Props>();

</script>

<template>
    <li class="notification-item">
        <div :class="`w-3rem h-3rem flex align-items-center justify-content-center bg-${props.notification.color}-100 border-circle mr-3 flex-shrink-0`">
            <i :class="`pi ${props.notification.icon} text-xl text-${props.notification.color}-500`" />
        </div>
        <span class="text-900 line-height-3"> {{ props.notification.message }} </span>
    </li>
</template>