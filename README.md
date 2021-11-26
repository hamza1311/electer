# Electer

A voting app to allow users to vote on, given multiple choices. It uses [`axum`](https://github.com/tokio-rs/axum) on
the backend and [`Svelte`](https://svelte.dev/) on the frontend using [`Vite`](https://vitejs.dev/) build tool. It
uses `PostgreSQL` as the database.

## Hosting

You can build this app in 2 ways:

1. Using the docker image
2. Manually building the app

### Building the docker image

Pull the published docker image:

```shell
docker pull ghcr.io/hamza1311/electer:latest
```

Once the pull completes, you can run the image as:

```shell
docker run -d --network host --name electer --env "DATABASE_URL=YOUR_POSTGRES_INSTANCE" --env "PORT=PORT_TO_RUN_ON" ghcr.io/hamza1311/electer
```

### Manually building the app

In order to build the app manually, following tools must be installed:

- [Rust](https://www.rust-lang.org)
- [Node JS](https://nodejs.org/)

#### Building frontend

```shell
cd frontend
npm install
npm run build
```

#### Building backend

```shell
cargo build --release
```

### Configuration

| Name           | Required   | Description                                                              | Default          |
|----------------|------------|--------------------------------------------------------------------------|------------------|
| `DATABASE_URL` | ✅          | The path at which your instance of  `PostgreSQL` is running              |                 |
| `PORT`         | ❌          | The port to run the server on                                            | 8000            |
| `DIST_DIR`     | ❌          | The path where frontend static files are (must **not** be set in docker) | `frontend/dist` |

## Contributions

Your contributions are welcome.
