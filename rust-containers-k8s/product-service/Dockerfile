FROM rust:latest AS builder
WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY inventory-service/Cargo.toml ./inventory-service/
COPY notification-service/Cargo.toml ./notification-service/
COPY order-service/Cargo.toml ./order-service/
COPY product-service/Cargo.toml ./product-service/
RUN for s in inventory notification order product;do mkdir $s-service/src && echo 'fn main() {}' > $s-service/src/main.rs; done
RUN cargo build --release --package product-service

# build the app
COPY . .
RUN cargo build --release --package product-service
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser

FROM debian:bookworm-slim
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
WORKDIR /app
COPY --from=builder /app/target/release/product-service ./
USER appuser:appuser
CMD ["/app/product-service"]
