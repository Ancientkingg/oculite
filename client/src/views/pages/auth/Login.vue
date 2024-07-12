<script setup>
import { useLayout } from '@/layout/composables/layout';
import { ref, computed } from 'vue';
import AppConfig from '@/layout/AppConfig.vue';

const { layoutConfig } = useLayout();
const password = ref('');

const logoUrl = computed(() => {
    return `/layout/images/${layoutConfig.darkTheme.value ? 'logo-white' : 'logo-dark'}.png`;
});
</script>

<template>
    <div class="surface-ground flex align-items-center justify-content-center min-h-screen min-w-screen overflow-hidden">
        <div class="flex flex-column align-items-center justify-content-center">
            <img
                :src="logoUrl"
                alt="Oculite logo"
                class="mb-5 w-6rem flex-shrink-0"
                >
            <div style="border-radius: 56px; padding: 0.3rem; background: linear-gradient(180deg, var(--primary-color) 10%, rgba(33, 150, 243, 0) 30%)">
                <div
                    class="w-full surface-card py-8 px-5 sm:px-8"
                    style="border-radius: 53px"
                    >
                    <div class="text-center mb-5">
                        <div class="text-900 text-3xl font-medium mb-3">
                            Welcome
                        </div>
                        <span class="text-600 font-medium">Please enter password to continue</span>
                    </div>

                    <div>
                        <label
                            for="password1"
                            class="block text-900 font-medium text-xl mb-2"
                            >Password</label>
                        <Password
                            id="password1"
                            v-model="password"
                            placeholder="Password"
                            :feedback="false"
                            :toggle-mask="true"
                            class="w-full mb-3"
                            input-class="w-full"
                            :input-style="{ padding: '1rem' }"
                            />
                        <Button
                            label="Sign In"
                            class="w-full p-3 text-xl"
                            />
                    </div>
                </div>
            </div>
        </div>
    </div>
    <AppConfig simple />
</template>

<style scoped>
.pi-eye {
    transform: scale(1.6);
    margin-right: 1rem;
}

.pi-eye-slash {
    transform: scale(1.6);
    margin-right: 1rem;
}
</style>
