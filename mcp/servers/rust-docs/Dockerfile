# ── Stage 1 : Build mcp-rust-docs (musl = static binary, no GLIBC dependency) ─
# Build context is mcp/ (parent directory)
FROM rust:1.94-slim AS rust-builder

RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY servers/rust-docs/Cargo.toml servers/rust-docs/Cargo.lock ./
COPY servers/rust-docs/src/ src/

RUN cargo build --release --target x86_64-unknown-linux-musl

# ── Stage 2 : Build rust-docs-mcp (nightly via rustup) ───────────────────────
FROM debian:bookworm-slim AS rdmcp-builder

RUN apt-get update && apt-get install -y curl pkg-config libssl-dev git ca-certificates \
    build-essential && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install rust-docs-mcp

# ── Stage 3 : Fetch & process docs (config-driven from manifest) ──────────────
FROM node:20-slim AS docs-builder

RUN apt-get update && apt-get install -y git curl && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# npm deps (cached layer)
COPY servers/rust-docs/package.json ./
RUN npm install --silent

# Manifest (source of truth) + processing scripts
COPY servers-manifest.json ./
COPY servers/rust-docs/fetch-docs.mjs ./
COPY servers/rust-docs/strip-mdx.mjs ./
COPY servers/rust-docs/generate-catalog.mjs ./
COPY servers/rust-docs/split-daisyui.mjs ./

# Local docs (referenced by type:"local" entries in manifest)
COPY servers/rust-docs/local-docs/ /local-docs/

# Fetch all doc sources defined in the manifest
RUN node fetch-docs.mjs

# ── Stage 4 : Runtime (nightly required for rustdoc JSON generation) ──────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates wget git curl build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install rustup + nightly at runtime (needed by rust-docs-mcp to run rustdoc)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

COPY --from=rust-builder  /app/target/x86_64-unknown-linux-musl/release/mcp-rust-docs  /usr/local/bin/
COPY --from=rdmcp-builder /root/.cargo/bin/rust-docs-mcp           /usr/local/bin/
COPY --from=docs-builder  /docs/                                    /docs/

EXPOSE 80
ENV DOCS_PATH=/docs
ENV RUST_DOCS_MCP_CACHE_DIR=/cache
ENV RUST_DOCS_MCP_TOOLCHAIN=nightly

CMD ["mcp-rust-docs"]
