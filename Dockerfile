# ---- Builder stage ----
FROM rust:1-bullseye AS builder
WORKDIR /app
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs
COPY Cargo.toml Cargo.toml
RUN cargo fetch
COPY . .
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/lucide-offline-cli /usr/local/bin/lucide-offline-cli
COPY icons /usr/local/share/lucide-offline-cli/icons
ENV LUCIDE_ICONS_DIR=/usr/local/share/lucide-offline-cli/icons
ENTRYPOINT ["lucide-offline-cli"]
CMD ["list"]
