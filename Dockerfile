# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1-slim AS chef
WORKDIR app
RUN apt update && apt install -y curl
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && apt install -y nodejs
RUN npm i -g yarn

ENV TRUNK_VERSION="v0.16.0"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt update && apt install -y pkg-config libssl-dev libpq-dev wget
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN yarn install --frozen-lockfile
RUN yarn tailwind:build
RUN (cd web && RUSTFLAGS=--cfg=web_sys_unstable_apis ../trunk build --release --public-url static)
RUN RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y libpq-dev libssl-dev
WORKDIR app
EXPOSE 3000
COPY --from=builder /app/target/release/urllb /usr/local/bin
ENTRYPOINT ["/usr/local/bin/urllb"]
