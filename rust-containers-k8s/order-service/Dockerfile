FROM rust:latest AS builder
WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY inventory-service/Cargo.toml ./inventory-service/
COPY notification-service/Cargo.toml ./notification-service/
COPY order-service/Cargo.toml ./order-service/
COPY product-service/Cargo.toml ./product-service/
RUN for s in inventory notification order product;do mkdir $s-service/src && echo 'fn main() {}' > $s-service/src/main.rs; done
RUN cargo build --release --package order-service

# build the app
COPY . .
RUN cargo build --release --package order-service
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser

FROM debian:bookworm-slim
RUN apt-get update && apt-get install libssl3 ca-certificates -y && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
WORKDIR /app
COPY --from=builder /app/target/release/order-service ./
USER appuser:appuser
CMD ["/app/order-service"]
