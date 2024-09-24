import express, { Express, Request, Response } from 'express';

const app: Express = express();
app.use(express.json());
const port = 3000;

app.post('/', (req: Request, res: Response) => {
	console.log(`Config: ${req.body.config}`);
	res.send({
		data: [
			{
				id: 2, // Unique identifier of the item (within the monitor)
				name: 'Toyota Avensis Remmen',
				currency: '€',
				icon: 'https://www.remdiscounter.nl/media/products/remschijven-toyota-avensis-stationwagen-t27-18-zrt271-30881_17608.jpg',
				link: 'https://google.com',
				price_data: 41.0,
			},
		],
	});
});

app.listen(port, () => {
	console.log(`[server]: Server is running at http://localhost:${port}`);
});
