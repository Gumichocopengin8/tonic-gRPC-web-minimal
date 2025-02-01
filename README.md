# gRPC-Web + Tonic + Tonic-Web

## How to Run

### Without Envoy

#### Server

- In `server` folder, `cargo run`

#### Client

- In `client` folder, `npm i && npm run dev`
- Access to `http://localhost:5173` and open the browser console and network tabs to see connections

### With Envoy

#### Server

- In root dir, `docker compose up`

#### Client

- In `client` folder, `npm i && npm run dev`
- Access to `http://localhost:5173` and open the browser console and network tabs to see connections
