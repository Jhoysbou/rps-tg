# Rock Paper Scissors telegram mini app
Multiplayer realtime game with matchmaking implemented with actor framework.

[Let me try](https://t.me/rpsMiniAppBot)

The repository contains a backend and a frontend projects.

Backend is written using Rust with [Actix and Actix Web](https://actix.rs).
Frontend — [Svelte](https://svelte.dev) without server-side rendering.
## Build and run
### Backend
You need to have rust and cargo installed.
[How to install rust and cargo?](https://www.rust-lang.org/tools/install)

```bash
cd server
```
Run the backend
```bash
cargo run
```
This command will download, compile all the dependencies alongside the source code and run it.

The server will be accessible on port `8080`. It has one websocket endpoint `/ws/{userId}`
All communication comes through the websocket connection.

### Frontend
You need to have node 18+ installed.
[How to install nodejs?](https://nodejs.org/en/download/package-manager)
```bash
cd client
```

Install dependencies
```bash
npm i
```

Start the development server
```bash
npm run dev
```

The frontend will be accessible on port `5173`.

## Documentation
[Frontend](/client/README.md) \
[Backend](/server/README.md)
