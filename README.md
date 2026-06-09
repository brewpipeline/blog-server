# blog-server

[![Deploy on Railway](https://railway.com/button.svg)](https://railway.com/deploy/rust-blog?referralCode=ajW30i&utm_medium=integration&utm_source=template&utm_campaign=generic)

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
when the `ssr` feature is enabled.

`blog-ui` is wired in as a Cargo workspace member at `blog-server/blog-ui`, but
it is **not** committed here (it is gitignored). The `Makefile` clones it for you
on demand, pinned to `BLOG_UI_TAG`:

```bash
make blog-ui            # clone blog-ui at BLOG_UI_TAG into ./blog-ui
make blog-ui BLOG_UI_TAG=1.4.2   # override the pinned tag
```

`make run` and `make build` depend on this target, so the clone happens
automatically the first time you build (see [Running](#running)). The clone is
idempotent — to switch tags after a clone exists, `rm -rf blog-ui` first.

## Workspace Layout

This repository is a Cargo workspace made up of several crates:

| Crate | Description |
| ----- | ----------- |
| `blog-server-api` | HTTP entry point, request routing and middleware setup. |
| `blog-server-services` | Service layer with implementations for posts, comments, authors and social integrations. |
| `blog-generic` | Shared domain entities, events and utilities used by other crates. |
| `blog-ui` | Yew/WASM front-end (`ssr` rendering + static assets). Fetched on demand via `make`; gitignored, not committed here. See [UI Repository](#ui-repository). |

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
* `git` and `make` (used to fetch the `blog-ui` workspace member)
* PostgreSQL database
* Optional: RabbitMQ for the event bus
* Optional, only to build the front-end / SSR assets: the
  `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
  and [`trunk`](https://trunkrs.dev/) (`cargo install trunk`)

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
make run
```

`make run` first fetches `blog-ui` (if missing) and then runs
`cargo run -p blog-server-api`. Use `make build` to compile without running.
You can still invoke cargo directly (`cargo run -p blog-server-api`), but only
once `blog-ui` has been cloned — otherwise the workspace fails to load.

On start up the server applies database migrations, connects to RabbitMQ if
available and listens on `SERVER_ADDRESS`.

> **Note:** server-side rendering reads `dist/index.html` at runtime (produced by
> a `trunk build` of `blog-ui`). A plain `make run` from the workspace root has no
> `dist/`, so SSR requests will fail locally; the full stack is assembled by the
> `Dockerfile`.

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

Fetch `blog-ui` first (so the workspace loads), then run the suite. CI scopes
tests to the server crate:

```bash
make blog-ui
cargo test -p blog-server-api
```

## License

Licensed under the [MIT license](LICENSE).

## Contributing

Issues and pull requests are welcome.  Please open an issue describing the change
or bug before submitting a pull request.
