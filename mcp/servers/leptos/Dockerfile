# Stage 1 : compiler le MCP server Rust
FROM rust:1.94-slim AS builder

RUN cargo install --git https://github.com/kneiht/leptos-mcp-server --locked

# Stage 2 : image finale Node.js + binaire Rust
FROM node:24-slim

RUN npm install -g supergateway

COPY --from=builder /usr/local/cargo/bin/leptos-mcp-server /usr/local/bin/

EXPOSE 80
CMD ["supergateway", "--stdio", "leptos-mcp-server", "--port", "80", "--host", "0.0.0.0", "--ssePath", "/leptos/sse", "--messagePath", "/leptos/message"]
