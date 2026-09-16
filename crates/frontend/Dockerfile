FROM rust:bookworm AS builder

WORKDIR /usr/src/app

# Install wasm32 target and prebuilt trunk for rapid builds
RUN rustup target add wasm32-unknown-unknown && \
    curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C /usr/local/bin

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

WORKDIR /usr/src/app/crates/frontend
RUN trunk build --release

FROM nginx:alpine

COPY crates/frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /usr/src/app/crates/frontend/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
