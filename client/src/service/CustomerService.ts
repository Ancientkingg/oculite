export class CustomerService {
    async getCustomersSmall() {
        const res = await fetch('/demo/data/customers-small.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }
    async getCustomersMedium() {
        const res = await fetch('/demo/data/customers-medium.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getCustomersLarge() {
        const res = await fetch('/demo/data/customers-large.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }
}
