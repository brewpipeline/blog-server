FROM rust:1.95-slim AS ui-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

ARG FEATURES="telegram,chatgpt,lang_ru"
ARG API_URL
ARG TITLE
ARG DESCRIPTION
ARG KEYWORDS
ARG ACCORDION_JSON
ARG TELEGRAM_BOT_LOGIN
ARG YANDEX_CLIENT_ID

ENV API_URL=$API_URL \
    TITLE=$TITLE \
    DESCRIPTION=$DESCRIPTION \
    KEYWORDS=$KEYWORDS \
    ACCORDION_JSON=$ACCORDION_JSON \
    TELEGRAM_BOT_LOGIN=$TELEGRAM_BOT_LOGIN \
    YANDEX_CLIENT_ID=$YANDEX_CLIENT_ID

WORKDIR /app/blog-ui
COPY blog-ui .
RUN trunk build --release -- --no-default-features --features "hydration,$FEATURES"

FROM rust:1.95-slim AS server-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

ARG FEATURES="telegram,chatgpt,lang_ru"
ARG API_URL
ARG TITLE
ARG DESCRIPTION
ARG KEYWORDS
ARG ACCORDION_JSON
ARG TELEGRAM_BOT_LOGIN
ARG YANDEX_CLIENT_ID

ENV API_URL=$API_URL \
    TITLE=$TITLE \
    DESCRIPTION=$DESCRIPTION \
    KEYWORDS=$KEYWORDS \
    ACCORDION_JSON=$ACCORDION_JSON \
    TELEGRAM_BOT_LOGIN=$TELEGRAM_BOT_LOGIN \
    YANDEX_CLIENT_ID=$YANDEX_CLIENT_ID

WORKDIR /app
COPY . .

RUN cargo build -p blog-server-api --release --no-default-features --features "ssr,$FEATURES"

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=server-builder /app/target/release/blog-server-api .
COPY --from=server-builder /app/config.yaml .
COPY --from=server-builder /app/index.html .
COPY --from=ui-builder /app/blog-ui/dist ./dist

CMD ["./blog-server-api"]
