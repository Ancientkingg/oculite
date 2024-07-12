export class CountryService {
    async getCountries() {
        const res = await fetch('/demo/data/countries.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }
}
