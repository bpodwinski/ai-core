# Template pour MCP communautaire via supergateway
# Adapter : npm package name, ssePath, messagePath

FROM node:24-slim

RUN npm install -g supergateway @upstash/context7-mcp

EXPOSE 80
CMD ["supergateway", "--stdio", "context7-mcp", "--port", "80", "--host", "0.0.0.0", "--ssePath", "/context7/sse", "--messagePath", "/context7/message"]
