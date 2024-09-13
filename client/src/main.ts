import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import PrimeVue, { usePrimeVue } from 'primevue/config';
import BadgeDirective from 'primevue/badgedirective';
import ConfirmationService from 'primevue/confirmationservice';
import DialogService from 'primevue/dialogservice';
import Ripple from 'primevue/ripple';
import StyleClass from 'primevue/styleclass';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';
import { VueQueryPlugin } from '@tanstack/vue-query';
import '@/assets/styles.scss';
import { useLayout } from './layout/composables/layout';

const app = createApp(App);

app.use(router);
app.use(PrimeVue, { ripple: true });
app.use(ToastService);
app.use(DialogService);
app.use(ConfirmationService);

app.use(VueQueryPlugin, {
    enableDevtoolsV6Plugin: true,
});

app.directive('tooltip', Tooltip);
app.directive('badge', BadgeDirective);
app.directive('ripple', Ripple);
app.directive('styleclass', StyleClass);

app.mount('#app');

app.runWithContext(() => {
    const getPreferredScheme = () =>
        window?.matchMedia?.('(prefers-color-scheme:dark)')?.matches
            ? 'dark'
            : 'light';

    const $primevue = usePrimeVue();
    const { layoutConfig } = useLayout();
    const theme = layoutConfig.theme.value
        .split(/dark|light/)
        .join(getPreferredScheme());

    $primevue.changeTheme(layoutConfig.theme.value, theme, 'theme-css', () => {
        layoutConfig.theme.value = theme;
    });
});
