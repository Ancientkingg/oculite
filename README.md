# Oculite

A user-friendly dashboard for monitoring prices and values.

> A personal write up on reverse-engineering the AutoDoc API can be found [here](/monitors/autodoc/README.md).

<img alt="Oculite dashboard" src="/@meta/images/dashboard.png">
<img alt="Oculite monitor" src="/@meta/images/category.png">
<img alt="Oculite dashboard on mobile" src="/@meta/images/dashboard_mobile.png" height="768">

## Tech stack

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)
- [Vue.js](https://vuejs.org/)
- [PrimeVue](https://primevue.org/)
- [Sakai](https://github.com/primefaces/sakai-vue/)

## How to run

### Server

To start the server run the following command:

```bash
cargo run // runs the server in debug release
```

### Client

The client can be started using the Vite development server. To start the client run the following command:

```bash
npm run dev
```

Optionally, the client can be built and automatically 'deployed' to the back-end by running the following command:

```bash
npm run deploy // builds the front-end and puts the files in the back-end's `public` directory to be served
```

### Monitor

To run the monitors, navigate to the `monitors` directory and run the following command:

```bash
npm run start
```

## Required Environment

### Server

- `DATABASE_URL`

### Client

- `VITE_API_BASE_URL`

### AutoDoc Monitor

- `API_BASE_URL`
- `API_TOKEN`

## How it works

Oculite was developed with decoupling in mind. The application consists of three main parts: the front-end, the back-end and the third party monitor(s). To display the time series data, the front-end communicates with the back-end. The back-end does all of the data processing and more importantly the data fetching.

The back-end is, in short, a data aggregation platform. It periodically fetches data from (third party) monitors, processes it and stores it in a database. The monitor services are responsible for extracting the data from the sources and providing it to the back-end in an agreed-upon format. This allows the monitor services to be decoupled from the back-end and allows third-party developers to write them in any programming language. All parts of the application are connected through a REST API.

## Front-end

The front-end is a Vue.js application that displays the data in a user-friendly way. It uses PrimeVue as a component library and Sakai as a design system. The front-end communicates with the back-end through a REST API.

## Back-end

The back-end is a Rocket application written in Rust. It is responsible for fetching and processing the data. It uses a PostgreSQL database to store the data. The back-end is also responsible for the authentication and authorization of the users.

## Monitor(s)

The monitor services are responsible for extracting the data from the sources (e.g., web scrapers) and providing it to the back-end.

Since retrieving data from the sources can be very specific to the source, the monitor services were introduced to abstract the data fetching process and provide a standardized format to the data. They communicate with the back-end by exposing a REST API.

## How to add a monitor

A monitor needs to expose a single REST API endpoint at `/` that returns data in the following JSON format:

```json
{
    "data": [
        {
            "id": 12345, // Unique identifier of the item (within the monitor)
            "name": "Name",
            "currency": "€",
            "icon": "icon",
            "link": "https://google.com",
            "price_data": 40.0
        }
    ]
}
```

When Oculite calls the monitor's API, it supplies a request body **TO** the monitor with the following format:

```json
interface RequestBody {
  config: string | null;
}
```

The request body can optionally contain the user's defined config for the monitor. The monitor can use this config to customize the data fetching process.

The JSON schema of the endpoint response **TO** Oculite should be as follows:

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "data": {
      "type": "array",
      "items": [
        {
          "type": "object",
          "properties": {
            "id": {
              "type": "integer"
            },
            "name": {
              "type": "string"
            },
            "currency": {
              "type": "string"
            },
            "icon": {
              "type": "string"
            },
            "link": {
              "type": "string"
            },
            "price_data": {
              "type": "number",
            }
          },
          "required": [
            "id",
            "name",
            "currency",
            "icon",
            "link",
            "price_data"
          ]
        }
      ]
    }
  },
  "required": [
    "data"
  ]
}
```

## How Oculite came to be

The whole idea of Oculite started back, when I went for my periodic vehicle inspection. It turned out that soon I would need new brakes. My dad at the time recommended me a website called [AutoDoc](https://autodoc.com). They sell car parts and accessories for a wide range of car brands. My dad mentioned to me that sometimes they would have discounts on their products, so he recommended to me to check their website from time to time. I, however, as a CSE student, thought that it would be a good idea to automate this process. I wanted to be able to see the price history of a product, and be able to get a notification when the price of a product drops. This is how Oculite came to be.

I realized this idea could be useful in other situations as well. Monitoring the price, or rather more abstractly, the value of a particular item, could be useful in many situations. For example, monitoring the price of a stock, or monitoring the price of a product on Amazon. This is why I decided to make Oculite a more general tool, that can theoretically be used to monitor the price of any item on any website. Hence the choice of decoupling the data aggregation part of the project from the monitoring part. This way, in the future it would be easy for me (and maybe even for other developers), to add support for new sources to track items from.

## Example monitors

Two monitors are included in the project as examples. `monitor-example` is a monitor example that returns a static object to show how a monitor should be structured. `autodoc` is a monitor that fetches data from the AutoDoc website. Please read the [README.md](/monitors/autodoc/README.md) in the `autodoc` directory to read how the monitor works (quite interesting if I say so myself :D).
