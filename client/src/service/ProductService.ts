export class ProductService {
    async getProductsSmall() {
        const res = await fetch('/demo/data/products-small.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getProducts() {
        const res = await fetch('/demo/data/products.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getProductsMixed() {
        const res = await fetch('/demo/data/products-mixed.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getProductsWithOrdersSmall() {
        const res = await fetch('/demo/data/products-orders-small.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getProductsWithOrdersLarge() {
        const res = await fetch('/demo/data/products-orders.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }
}
