FROM rust:1.74-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/zero-trust-network .
EXPOSE 8080
CMD ["./zero-trust-network"]
