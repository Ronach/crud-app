FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.jspn

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM rust:1.72-slim AS quotes
COPY --from=builder /app/target/release/quotes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/quotes"]