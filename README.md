# blog-server

A modular blog platform written in Rust.  The project exposes a HTTP API for posts,
comments and author management while broadcasting events to Telegram and Discord.
The server is built on top of the [Screw](https://github.com/Tikitko/screw) ecosystem,
[Hyper](https://hyper.rs/) for the HTTP layer and [Rbatis](https://rbatis.github.io/rbatis.io)
for database access with PostgreSQL. The web front-end lives in the
[blog-ui](https://github.com/brewpipeline/blog-ui) repository.

## UI Repository

The user-facing website is developed separately in the
[`blog-ui`](https://github.com/brewpipeline/blog-ui) repository. It provides
the front-end that consumes this API and offers server-side rendering support
when the `ssr` feature is enabled. Check that repository for instructions on
building and running the web interface.

## Workspace Layout

This repository is a Cargo workspace made up of several crates:

| Crate | Description |
| ----- | ----------- |
| `blog-server-api` | HTTP entry point, request routing and middleware setup. |
| `blog-server-services` | Service layer with implementations for posts, comments, authors and social integrations. |
| `blog-generic` | Shared domain entities, events and utilities used by other crates. |

## Features

* CRUD operations for posts and comments.
* Author profiles with subscription management and optional social data.
* JSON based API with automatic request/response handling.
* Optional server‑side rendering through the [`blog-ui`](https://github.com/brewpipeline/blog-ui) crate (`--features ssr`).
* Optional login via Yandex or Telegram (`--features yandex` or `--features telegram`).
* Event broadcasting through RabbitMQ, Telegram bots and Discord webhooks.
* Configuration driven notifications via `config.yaml`.

## Requirements

* [Rust](https://www.rust-lang.org/) toolchain (1.70 or later recommended)
* PostgreSQL database
* Optional: RabbitMQ for the event bus
* Optional: [`blog-ui`](https://github.com/brewpipeline/blog-ui) repository when using server‑side rendering

## Configuration

Compilation relies on a few environment variables.  They must be supplied at build
or run time:

```bash
export SITE_URL="http://127.0.0.1:3000"
export JWT_SECRET="changeme"
export SERVER_ADDRESS="127.0.0.1:3000"
export PG_URL="postgres://postgres:postgres@localhost:5432/blog"
export RABBIT_URL="amqp://guest:guest@localhost:5672/"
export TELEGRAM_BOT_TOKEN="0000000:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

Runtime configuration such as Telegram chat IDs or Discord webhooks is stored in
`config.yaml`.

## Running

1. Ensure PostgreSQL is running and matches the `PG_URL` connection string.
2. Adjust `config.yaml` for notification channels if necessary.
3. Start the server:

```bash
cargo run -p blog-server-api
```

On start up the server applies database migrations, connects to RabbitMQ if
available and listens on `SERVER_ADDRESS`.

## API Overview

The router exposes a JSON API under the `/api` path.  A selection of routes:

* `GET /api/posts` – list published posts
* `GET /api/post/{id}` – retrieve a single post
* `POST /api/post` – create a post
* `PATCH /api/post/{id}` – update a post
* `DELETE /api/post/{id}` – delete a post
* `GET /api/comments/{post_id}` – list comments for a post
* `POST /api/comment` – create a comment
* `GET /api/author/me` – current author profile
* `POST /api/login` – password based login

Many additional endpoints handle author management, subscriptions and searching.
See `router.rs` for the full list.

## Testing

Run the complete test suite for all workspace members with:

```bash
cargo test
```

## License

Licensed under the [MIT license](LICENSE).

## Contributing

Issues and pull requests are welcome.  Please open an issue describing the change
or bug before submitting a pull request.
