# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM rust:1-slim AS builder
WORKDIR app
RUN apt update && apt install -y curl wget pkg-config libssl-dev libpq-dev
RUN rustup target add wasm32-unknown-unknown

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && apt install -y nodejs
RUN npm i -g yarn

ENV TRUNK_VERSION="v0.16.0"

# Install trunk
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


# Build application
COPY . .
RUN yarn install --frozen-lockfile
RUN yarn tailwind:build
RUN (cd web && RUSTFLAGS=--cfg=web_sys_unstable_apis ../trunk build --release --public-url static)
RUN (cd web/dist && gzip * -k)
RUN STATIC_DIR="/app/web/dist" RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release

RUN ls -la web/dist

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y libpq-dev libssl-dev
WORKDIR app
EXPOSE 3000
COPY --from=builder /app/target/release/urllb /usr/local/bin
COPY --from=builder /app/web/dist/* /app/web/dist/
RUN ls -la /app/web
RUN ls -la /app/web/dist
ENTRYPOINT ["/usr/local/bin/urllb"]
