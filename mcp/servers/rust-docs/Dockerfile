FROM rust:1.94-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev git && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/

RUN cargo build --release

# Cloner les docs des repos communautaires
RUN git clone --depth 1 https://github.com/synphonyte/leptos-use /tmp/leptos-use

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mcp-rust-docs /usr/local/bin/

# Docs custom (écrites à la main)
COPY docs/ /docs/

# Docs leptos-use (clonées depuis GitHub)
COPY --from=builder /tmp/leptos-use/docs/book/src/ /docs/leptos-use/

EXPOSE 80
ENV DOCS_PATH=/docs
CMD ["mcp-rust-docs"]
