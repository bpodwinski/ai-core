# ── Stage 1 : Rust ────────────────────────────────────────────────────────────
FROM rust:1.94-slim AS rust-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release

# ── Stage 2 : Node.js (traitement des docs) ───────────────────────────────────
FROM node:20-slim AS node-builder

RUN apt-get update && apt-get install -y git curl && rm -rf /var/lib/apt/lists/*

# Cloner les docs des repos communautaires
RUN git clone --depth 1 https://github.com/leptos-rs/book /tmp/leptos-book
RUN git clone --depth 1 https://github.com/synphonyte/leptos-use /tmp/leptos-use
RUN git clone --depth 1 https://github.com/rust-lang/book /tmp/rust-book
RUN curl -L https://daisyui.com/llms.txt -o /tmp/daisyui-docs.md && \
    wc -c /tmp/daisyui-docs.md
RUN git clone --depth 1 https://github.com/tailwindlabs/tailwindcss.com /tmp/tailwindcss-site

# Installer les dépendances npm (layer séparé pour le cache)
COPY package.json /tmp/package.json
COPY split-daisyui.mjs /tmp/split-daisyui.mjs
RUN cd /tmp && npm install --silent

# Traiter les docs TailwindCSS
COPY strip-mdx.mjs /tmp/strip-mdx.mjs
COPY generate-catalog.mjs /tmp/generate-catalog.mjs
RUN cd /tmp && \
    node strip-mdx.mjs /tmp/tailwindcss-site/src/docs/ && \
    node generate-catalog.mjs /tmp/tailwindcss-site/src/docs/catalog.md

# Découper le llms.txt DaisyUI en un fichier par composant
RUN node /tmp/split-daisyui.mjs /tmp/daisyui-docs.md /tmp/daisyui-split

# ── Stage 3 : Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates wget && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /app/target/release/mcp-rust-docs /usr/local/bin/

# Docs Leptos Book officiel
COPY --from=node-builder /tmp/leptos-book/src/             /docs/leptos/

# Docs leptos-use
COPY --from=node-builder /tmp/leptos-use/docs/book/src/    /docs/leptos-use/

# Docs Rust Book officiel
COPY --from=node-builder /tmp/rust-book/src/               /docs/rust/

# Docs DaisyUI : un fichier par composant/section
COPY --from=node-builder /tmp/daisyui-split/               /docs/daisyui/

# Docs TailwindCSS (MDX → Markdown propre + catalog des classes)
COPY --from=node-builder /tmp/tailwindcss-site/src/docs/   /docs/tailwindcss/

# Docs InduFlow (locales)
COPY induflow/ /docs/induflow/

EXPOSE 80
ENV DOCS_PATH=/docs
CMD ["mcp-rust-docs"]
