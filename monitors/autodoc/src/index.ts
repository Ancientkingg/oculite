require('dotenv').config();
import express, { Express, Request, Response } from 'express';

import { ApiResponse } from './ApiResponse';

const app: Express = express();
app.use(express.json());
const port = 3000;

if (!process.env.API_BASE_URL) {
	console.error('API_BASE_URL environment variable is not set');
	process.exit(1);
}

if (!process.env.API_TOKEN) {
	console.error('API_TOKEN environment variable is not set');
	process.exit(1);
}

const requestHeaders = new Headers();
requestHeaders.set('api-platform', 'android');
requestHeaders.set('api-project-country', '4');
requestHeaders.set('api-project-lang', '9');
requestHeaders.set('api-token', process.env.API_TOKEN!);
requestHeaders.set('api-version', '2.18.1');

const parseConfig = (config: string): string[] => {
	return config.split('\n').map((line) => line.trim().split('ridex/')[1]);
};

app.post('/', async (req: Request, res: Response) => {
	console.log(`Config: ${req.body.config}`);

    if (!req?.body?.config) {
        res.send({
            data: [],
        });
        return;
    }

	const productIds = parseConfig(req.body.config!);

	const apiResponses = productIds.map((id) =>
		fetch(process.env.API_BASE_URL! + id, {
			method: 'GET',
			headers: requestHeaders,
		})
	);

    const jsonResponses: ApiResponse[] = await Promise.all(apiResponses)
        .then((responses) => Promise.all(responses.map((res) => res.json()))) as ApiResponse[];

    const products = jsonResponses.map((response: ApiResponse) => ({
        id: response.data.id,
        name: `${response.data.brand} ${response.data.genericArticle}`,
        currency: '€',
        icon: response.data.images[0].small,
        link: response.data.websiteUrl,
        price_data: response.data.price.current.price,
    }));

	res.send({
		data: products,
	});
});

app.listen(port, () => {
	console.log(
		`[MONITOR]: AutoDoc monitor is running at http://localhost:${port}`
	);
});
