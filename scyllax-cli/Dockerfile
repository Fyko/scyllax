FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin scyllax-cli

FROM alpine AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/scyllax-cli .

ENTRYPOINT ["/app/scyllax-cli"]
