# Oculite

A user-friendly dashboard for monitoring prices and values.

## Tech stack

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)
- [Vue.js](https://vuejs.org/)
- [PrimeVue](https://primevue.org/)
- [Sakai](https://github.com/primefaces/sakai-vue/)

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

A monitor needs to expose a single REST API endpoint at `/` that returns the data in the following JSON format:

```json
{
    "data": [
        {
            "id": 12345, // Unique identifier of the item (within the monitor)
            "name": "Name",
            "currency": "€",
            "icon": "icon",
            "link": "https://google.com",
            "price_data": [40.0] // Usually this array is of max length of 1
        }
    ]
}
```

The JSON schema is as follows:

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
              "type": "array",
              "items": [
                {
                  "type": "number"
                }
              ]
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
