# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1-slim AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt update && apt install -y pkg-config libssl-dev libpq-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN (cd web && trunk build --release --public-url static)
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y libpq-dev libssl-dev
WORKDIR app
EXPOSE 3000
COPY --from=builder /app/target/release/urllb /usr/local/bin
ENTRYPOINT ["/usr/local/bin/urllb"]
