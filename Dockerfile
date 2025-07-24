FROM rust:1.88.0-bookworm AS builder

RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential
WORKDIR /app

COPY Cargo.toml ./
COPY apps ./apps
COPY crates ./crates
COPY tests ./tests
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

RUN useradd -m appuser
USER appuser

COPY --from=builder /app/target/release/api ./api
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./api"]
