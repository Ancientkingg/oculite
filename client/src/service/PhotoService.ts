export class PhotoService {
    async getImages() {
        const res = await fetch('/demo/data/photos.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }
}
