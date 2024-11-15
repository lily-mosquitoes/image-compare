FROM rust:1-slim-bookworm AS build
WORKDIR /srv/build

# deps
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# cache build
RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=$(pwd)/target/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=Trunk.toml,target=Trunk.toml \
    --mount=type=bind,source=tailwind.config.js,target=tailwind.config.js \
    --mount=type=bind,source=src,target=src \
    trunk build --release

FROM nginx:latest
COPY --from=build /srv/build/dist/ /srv/www/
# remove default.conf; any .conf needed should be in templates
RUN rm /etc/nginx/conf.d/default.conf
