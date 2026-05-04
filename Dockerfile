FROM rust:1.95-slim AS ui-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl git && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    | tar -xzf - -C /usr/local/bin && \
    cargo binstall --no-confirm trunk

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
COPY blog-server-api/Cargo.toml blog-server-api/Cargo.toml
RUN BLOG_UI_TAG=$(sed -n '/\[dependencies\.blog-ui\]/,/^\[/p' blog-server-api/Cargo.toml | grep '^tag = ' | sed 's/tag = "\(.*\)"/\1/') && \
    git clone --depth 1 --branch "$BLOG_UI_TAG" https://github.com/Tikitko/blog-ui.git /app/blog-ui

WORKDIR /app/blog-ui
RUN trunk build --release --no-default-features --features "hydration,$FEATURES"

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
COPY --from=ui-builder /app/blog-ui /app/blog-ui
COPY --from=ui-builder /app/blog-ui/dist/index.html ./index.html

RUN printf '\n[patch."https://github.com/Tikitko/blog-ui.git"]\nblog-ui = { path = "blog-ui" }\n' >> Cargo.toml

RUN find /app -maxdepth 2 -name "Cargo.toml"
RUN cargo build -p blog-server-api --release --no-default-features --features "ssr,$FEATURES"

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 nginx gettext-base && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=server-builder /app/target/release/blog-server-api .
COPY --from=server-builder /app/config.yaml .
COPY --from=server-builder /app/index.html .
COPY --from=ui-builder /app/blog-ui/dist ./dist
COPY nginx.conf.template /etc/nginx/nginx.conf.template

RUN rm /etc/nginx/sites-enabled/default

RUN printf '#!/bin/sh\n\
envsubst "${PORT}" < /etc/nginx/nginx.conf.template > /etc/nginx/sites-enabled/default\n\
./blog-server-api &\n\
exec nginx -g "daemon off;"\n' > /app/start.sh && chmod +x /app/start.sh

CMD ["/app/start.sh"]
