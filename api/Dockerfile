FROM rust:1-bookworm AS builder
WORKDIR /usr/src/app
COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations
COPY tests ./tests
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/platform-api /usr/local/bin/platform-api
EXPOSE 8080
CMD ["platform-api"]
