import { ToastServiceMethods } from 'primevue/toastservice';

export const showToast = (
    toast: ToastServiceMethods,
    severity:
        | 'success'
        | 'info'
        | 'warn'
        | 'error'
        | 'secondary'
        | 'contrast'
        | undefined,
    summary: string,
    detail: string,
    life: number,
) => {
    toast.add({ severity, summary, detail, life });
};
