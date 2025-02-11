####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

# Create appuser
ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --release --package order-service

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install libssl3 ca-certificates -y && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/order-service ./

# Use an unprivileged user.
USER appuser:appuser

CMD ["/app/order-service"]
