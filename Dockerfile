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
COPY --from=builder /app/target/release/lucide-svg-rs /usr/local/bin/lucide-svg-rs
COPY icons /usr/local/share/lucide-svg-rs/icons
ENV LUCIDE_ICONS_DIR=/usr/local/share/lucide-svg-rs/icons
ENTRYPOINT ["lucide-svg-rs"]
CMD ["list"]
