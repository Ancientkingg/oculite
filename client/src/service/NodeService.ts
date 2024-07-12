export class NodeService {
    async getFiles() {
        const res = await fetch('/demo/data/files.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getLazyFiles() {
        const res = await fetch('/demo/data/files-lazy.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getFilesystem() {
        const res = await fetch('/demo/data/filesystem.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getLazyFilesystem() {
        const res = await fetch('/demo/data/filesystem-lazy.json', {
            headers: { 'Cache-Control': 'no-cache' },
        });
        const d = await res.json();
        return d.data;
    }

    async getTreeTableNodes() {
        const res = await fetch('/demo/data/treetablenodes.json');
        const d = await res.json();
        return d.root;
    }

    async getTreeNodes() {
        const res = await fetch('/demo/data/treenodes.json');
        const d = await res.json();
        return d.root;
    }
}
