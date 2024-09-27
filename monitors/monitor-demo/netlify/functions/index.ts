import express, { Express, Request, Response } from 'express';
import serverless from "serverless-http";
import getItems from '../../sim/items';


const app: Express = express();

app.post('/', (_: Request, res: Response) => {
	res.send({
		data: getItems(Date.now()),
	});
});

export const handler = serverless(app);