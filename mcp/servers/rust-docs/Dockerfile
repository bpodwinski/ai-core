FROM rust:1.94-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev git curl nodejs npm && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/

RUN cargo build --release

# Cloner les docs des repos communautaires
RUN git clone --depth 1 https://github.com/leptos-rs/book /tmp/leptos-book
RUN git clone --depth 1 https://github.com/synphonyte/leptos-use /tmp/leptos-use
RUN git clone --depth 1 https://github.com/rust-lang/book /tmp/rust-book
RUN curl -L https://daisyui.com/llms.txt -o /tmp/daisyui-docs.md
RUN git clone --depth 1 https://github.com/tailwindlabs/tailwindcss.com /tmp/tailwindcss-site
COPY strip-mdx.mjs /tmp/strip-mdx.mjs
COPY generate-catalog.cjs /tmp/generate-catalog.cjs
RUN echo '{"type":"module"}' > /tmp/package.json && \
    cd /tmp && npm install --save --silent \
      remark remark-mdx unist-util-visit unist-util-remove \
      tailwindcss@3 postcss && \
    node strip-mdx.mjs /tmp/tailwindcss-site/src/docs/ && \
    node generate-catalog.cjs /tmp/tailwindcss-site/src/docs/catalog.md

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mcp-rust-docs /usr/local/bin/

# Docs Leptos Book officiel
COPY --from=builder /tmp/leptos-book/src/ /docs/leptos/

# Docs leptos-use
COPY --from=builder /tmp/leptos-use/docs/book/src/ /docs/leptos-use/

# Docs Rust Book officiel
COPY --from=builder /tmp/rust-book/src/ /docs/rust/

# Docs DaisyUI optimisées pour les assistants IA
COPY --from=builder /tmp/daisyui-docs.md /docs/daisyui/reference.md

# Docs InduFlow (locales)
COPY induflow/ /docs/induflow/

# Docs TailwindCSS (194 pages, MDX converti en Markdown propre par remark)
COPY --from=builder /tmp/tailwindcss-site/src/docs/ /docs/tailwindcss/

EXPOSE 80
ENV DOCS_PATH=/docs
CMD ["mcp-rust-docs"]
