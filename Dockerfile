# Set up cargo chef and compiling for WASM
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /lexor
RUN rustup target add wasm32-unknown-unknown \
    && curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

# Generate cargo chef recipe
FROM chef AS prepare
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build backend targeting WASM
FROM chef AS backend-builder
COPY --from=prepare /lexor/recipe.json recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown \
    --recipe-path recipe.json --package lexor-api
COPY . .
RUN wasm-pack build --release --target web \
    --out-dir /lexor/frontend/combinator-explorer/src/wasm \
    frontend/lexor-api

# Build frontend
FROM node:26-alpine AS frontend-builder
WORKDIR /web
COPY frontend/combinator-explorer/package*.json ./
RUN npm ci

COPY frontend/combinator-explorer ./
COPY --from=backend-builder /lexor/frontend/combinator-explorer/src/wasm \
     ./src/wasm

RUN npm run build

# Serve
FROM caddy:2-alpine AS runtime
COPY --from=frontend-builder /web/dist /serve

RUN echo -e ":5173 {\n\
    root * /serve\n\
    encode zstd gzip\n\
    try_files {path} /index.html\n\
    file_server\n\
}" > /etc/caddy/Caddyfile

USER 10001:10001
EXPOSE 5173

CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]
