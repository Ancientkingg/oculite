import express, { Express, Request, Response } from "express";

const app: Express = express();
const port = 3000;

app.get("/", (req: Request, res: Response) => {
  res.send({
    "data": [
        {
            "id": 1, // Unique identifier of the item (within the monitor)
            "name": "Toyota Avensis Remmen",
            "currency": "€",
            "icon": "https://www.remdiscounter.nl/media/products/remschijven-toyota-avensis-stationwagen-t27-18-zrt271-30881_17608.jpg",
            "link": "https://google.com",
            "price_data": 41.0 // Usually this array is of max length of 1
        }
    ]
});
});

app.listen(port, () => {
  console.log(`[server]: Server is running at http://localhost:${port}`);
});