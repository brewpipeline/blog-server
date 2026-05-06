# syntax=docker/dockerfile:1
FROM rust:1.95-slim AS ui-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl git && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    | tar -xzf - -C /usr/local/bin && \
    cargo binstall --no-confirm trunk

ARG FEATURES="telegram,chatgpt,lang_ru"
ARG DOMAIN
ARG TITLE
ARG DESCRIPTION
ARG KEYWORDS
ARG ACCORDION_JSON
ARG TELEGRAM_BOT_LOGIN
ARG YANDEX_CLIENT_ID
ARG LOGO_URL=""
ARG THEME=""
ARG COLOR_PRIMARY_BG=""
ARG COLOR_BODY_BG=""
ARG COLOR_SECOND_BG=""
ARG COLOR_LIGHT=""
ARG COLOR_BODY=""

ENV API_URL=https://$DOMAIN/api \
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

RUN set -e; \
    for css in dist/index-*.css; do \
      [ -n "${COLOR_PRIMARY_BG:-}" ] && sed -i "s/--bs-primary-bg-subtle:#242424;/--bs-primary-bg-subtle:${COLOR_PRIMARY_BG};/g; s/--bs-primary-bg-subtle: #242424;/--bs-primary-bg-subtle: ${COLOR_PRIMARY_BG};/g" "$css" || true; \
      [ -n "${COLOR_BODY_BG:-}" ]    && sed -i "s/--bs-body-bg:#141414;/--bs-body-bg:${COLOR_BODY_BG};/g; s/--bs-body-bg: #141414;/--bs-body-bg: ${COLOR_BODY_BG};/g" "$css" || true; \
      [ -n "${COLOR_SECOND_BG:-}" ]  && sed -i "s/--bs-second-bg:#1c1b1b;/--bs-second-bg:${COLOR_SECOND_BG};/g; s/--bs-second-bg: #1c1b1b;/--bs-second-bg: ${COLOR_SECOND_BG};/g" "$css" || true; \
      [ -n "${COLOR_LIGHT:-}" ]      && sed -i "s/--bs-light-color:#c6c7c8;/--bs-light-color:${COLOR_LIGHT};/g; s/--bs-light-color: #c6c7c8;/--bs-light-color: ${COLOR_LIGHT};/g" "$css" || true; \
      [ -n "${COLOR_BODY:-}" ]       && sed -i "s/--bs-body-color:#f2f2f2;/--bs-body-color:${COLOR_BODY};/g; s/--bs-body-color: #f2f2f2;/--bs-body-color: ${COLOR_BODY};/g" "$css" || true; \
    done; \
    [ -n "${COLOR_PRIMARY_BG:-}" ] && sed -i "s/content=\"#242424\"/content=\"${COLOR_PRIMARY_BG}\"/g" dist/index.html || true; \
    [ -n "${THEME:-}" ] && sed -i "s/data-bs-theme=\"dark\"/data-bs-theme=\"${THEME}\"/g; s/data-bs-theme=dark/data-bs-theme=${THEME}/g" dist/index.html || true; \
    [ -n "${LOGO_URL:-}" ] && curl -fsSL "${LOGO_URL}" -o dist/logo.svg || true

FROM rust:1.95-slim AS server-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev git && rm -rf /var/lib/apt/lists/*

ARG FEATURES="telegram,chatgpt,lang_ru"
ARG DOMAIN
ARG TITLE
ARG DESCRIPTION
ARG KEYWORDS
ARG ACCORDION_JSON
ARG TELEGRAM_BOT_LOGIN
ARG YANDEX_CLIENT_ID

ENV API_URL=https://$DOMAIN/api \
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

RUN cargo build -p blog-server-api --release --no-default-features --features "ssr,$FEATURES"

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 nginx gettext-base && rm -rf /var/lib/apt/lists/*
RUN rm -f /etc/nginx/sites-enabled/default \
          /etc/nginx/sites-available/default \
          /etc/nginx/conf.d/default.conf \
          /var/www/html/index.nginx-debian.html

ARG DOMAIN
ARG IMAGES_PROCESSOR_ADDRESS
ENV SERVER_ADDRESS="127.0.0.1:3000" \
    SITE_URL=https://$DOMAIN \
    IMAGES_PROCESSOR_ADDRESS=$IMAGES_PROCESSOR_ADDRESS

WORKDIR /app
COPY --from=server-builder /app/target/release/blog-server-api .
COPY --from=server-builder /app/config.yaml .
COPY --from=server-builder /app/index.html .
COPY --from=ui-builder /app/blog-ui/dist ./dist

COPY <<'EOF' /etc/nginx/conf.d/default.conf.template
server {
    listen 0.0.0.0:${PORT};

    root /app/dist;

    underscores_in_headers on;

    resolver ${RESOLVER} valid=10s;

    location / {
        try_files $uri @serverproxy;
    }

    location /images/external/ {
        set $images_upstream "${IMAGES_PROCESSOR_ADDRESS}";
        rewrite ^/images/external/(.*)$ /$1 break;
        proxy_pass http://$images_upstream;
        proxy_http_version 1.1;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
    }

    location @serverproxy {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
    }
}
EOF

COPY <<'EOF' /app/start.sh
#!/bin/sh
set -eu
echo "PORT=$PORT"
RESOLVER=$(awk '/^nameserver/ {print $2; exit}' /etc/resolv.conf)
case "$RESOLVER" in *:*) RESOLVER="[$RESOLVER]" ;; esac
echo "RESOLVER=$RESOLVER"
export PORT IMAGES_PROCESSOR_ADDRESS RESOLVER
envsubst '${PORT} ${IMAGES_PROCESSOR_ADDRESS} ${RESOLVER}' \
    < /etc/nginx/conf.d/default.conf.template \
    > /etc/nginx/conf.d/default.conf
nginx -t
./blog-server-api &
SERVER_PID=$!
( wait "$SERVER_PID"; echo "blog-server-api exited" >&2; kill 1 ) &
exec nginx -g "daemon off;"
EOF

RUN chmod +x /app/start.sh

CMD ["/app/start.sh"]
