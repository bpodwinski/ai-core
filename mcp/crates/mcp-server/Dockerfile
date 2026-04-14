# ── Stage 1 : Build mcp-server (musl = static binary, no GLIBC dependency) ─
# Build context is mcp/ (parent directory)
FROM rust:1.94-slim AS rust-builder

RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/mcp-common/ crates/mcp-common/
COPY crates/mcp-server/ crates/mcp-server/
# Workspace stubs — Cargo needs all member manifests even if we don't build them
COPY crates/mcp-build/Cargo.toml crates/mcp-build/Cargo.toml
COPY crates/mcp-configgen/Cargo.toml crates/mcp-configgen/Cargo.toml
COPY crates/mcp-hooks/Cargo.toml crates/mcp-hooks/Cargo.toml
RUN mkdir -p crates/mcp-build/src crates/mcp-configgen/src crates/mcp-hooks/src \
    && echo 'fn main(){}' > crates/mcp-build/src/main.rs \
    && echo 'fn main(){}' > crates/mcp-configgen/src/main.rs \
    && echo 'fn main(){}' > crates/mcp-hooks/src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl -p mcp-server

# ── Stage 2 : Build mcp-build (musl = static binary, no GLIBC dependency) ────
FROM rust:1.94-slim AS build-tools

RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/mcp-common/ crates/mcp-common/
COPY crates/mcp-build/ crates/mcp-build/
# Workspace stubs — Cargo needs all member manifests even if we don't build them
COPY crates/mcp-server/Cargo.toml crates/mcp-server/Cargo.toml
COPY crates/mcp-configgen/Cargo.toml crates/mcp-configgen/Cargo.toml
COPY crates/mcp-hooks/Cargo.toml crates/mcp-hooks/Cargo.toml
RUN mkdir -p crates/mcp-server/src crates/mcp-configgen/src crates/mcp-hooks/src \
    && echo 'fn main(){}' > crates/mcp-server/src/main.rs \
    && echo 'fn main(){}' > crates/mcp-configgen/src/main.rs \
    && echo 'fn main(){}' > crates/mcp-hooks/src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl -p mcp-build

# ── Stage 3 : Build rust-docs-mcp (nightly via rustup) ───────────────────────
FROM debian:bookworm-slim AS rdmcp-builder

RUN apt-get update && apt-get install -y curl pkg-config libssl-dev git ca-certificates \
    build-essential && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

# Fork with --features support for crates with mutually exclusive features
# Branch: feat/cache-crate-specific-features (not yet merged to main)
RUN cargo install --git https://github.com/bpodwinski/rust-docs-mcp.git --branch feat/cache-crate-specific-features rust-docs-mcp

# ── Stage 4 : Fetch & process docs using the mcp-build binary ────────────────
FROM debian:bookworm-slim AS docs-builder

RUN apt-get update && apt-get install -y git ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY --from=build-tools /app/target/x86_64-unknown-linux-musl/release/mcp-build /usr/local/bin/mcp-build

# Manifest (source of truth) + local docs referenced by type:"local" entries
COPY servers-manifest.json ./
COPY crates/mcp-server/local-docs/ /local-docs/

RUN mcp-build fetch-docs --manifest servers-manifest.json --docs-dir /docs --local-docs /local-docs --tmp /tmp/fetch

# ── Stage 5 : Runtime (nightly required for rustdoc JSON generation) ─────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates wget git curl build-essential pkg-config libssl-dev libclang-dev python3 \
    && rm -rf /var/lib/apt/lists/*

# Install rustup + nightly at runtime (needed by rust-docs-mcp to run rustdoc)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"
# Install the specific nightly version required by rust-docs-mcp (rustdoc-types 0.53 = format v53)
RUN rustup toolchain install nightly-2025-06-24 --profile minimal

COPY --from=rust-builder  /app/target/x86_64-unknown-linux-musl/release/mcp-rust-docs  /usr/local/bin/mcp-rust-docs
COPY --from=rdmcp-builder /root/.cargo/bin/rust-docs-mcp           /usr/local/bin/
COPY --from=docs-builder  /docs/                                    /docs/

EXPOSE 80
ENV DOCS_PATH=/docs
ENV RUST_DOCS_MCP_CACHE_DIR=/cache
ENV RUST_DOCS_MCP_TOOLCHAIN=nightly-2025-06-24
# Force nightly for all cargo invocations — overrides any project rust-toolchain.toml
# (e.g. sqlx pins 1.78 which can't handle edition2024 deps like time-0.3.47)
ENV RUSTUP_TOOLCHAIN=nightly-2025-06-24
# cfg flags pour les crates avec features unstable
# tokio (io-uring, taskdump) et reqwest (http3) nécessitent ces flags à la compilation
ENV RUSTFLAGS="--cfg tokio_unstable --cfg reqwest_unstable"
ENV RUSTDOCFLAGS="--cfg tokio_unstable --cfg reqwest_unstable"

CMD ["mcp-rust-docs"]
