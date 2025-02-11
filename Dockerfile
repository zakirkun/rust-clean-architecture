# Builder stage
FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libpq-dev && \
    cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust-clean-architecture /usr/local/bin/

EXPOSE 3000

CMD ["rust-clean-architecture"] 