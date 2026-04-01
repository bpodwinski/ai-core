# Plan : Serveur MCP personnel

## Objectif

Heberger ses propres MCP servers sur un serveur perso, accessibles via HTTPS/SSE depuis n'importe quel PC avec Claude Code. Plus besoin d'installer les binaires localement.

## Architecture

```
Claude Code (PC quelconque)
    │
    │ HTTPS + SSE
    ▼
Nginx (reverse proxy SSL, mcp.ton-domaine.com)
    │
    ├─ /mcp/leptos/    ──> supergateway :8090 ──stdio──> leptos-mcp-server
    ├─ /mcp/axum/      ──> supergateway :8091 ──stdio──> axum-mcp-server
    ├─ /mcp/tailwind/  ──> supergateway :8092 ──stdio──> tailwind-mcp-server
    └─ /mcp/sqlx/      ──> supergateway :8093 ──stdio──> sqlx-mcp-server
```

**Composants cles :**
- **supergateway** : bridge HTTP/SSE ↔ stdio (transforme un MCP stdio en MCP remote)
- **Nginx** : reverse proxy HTTPS, termine le SSL, desactive le buffering pour SSE
- **Docker Compose** : orchestre tous les containers
- **Let's Encrypt** : certificat SSL gratuit

## Etapes

### Phase 1 : Creer le repo Git

```
mcp-servers/
├── docker-compose.yml
├── nginx/
│   └── mcp.conf
├── servers/
│   ├── leptos/
│   │   └── Dockerfile
│   ├── axum/
│   │   └── Dockerfile
│   └── (autres frameworks)/
│       └── Dockerfile
├── scripts/
│   └── add-server.sh        ← Script helper pour ajouter un nouveau MCP
└── README.md
```

### Phase 2 : Dockerfile type

Chaque MCP server a son propre Dockerfile multi-stage :

```dockerfile
FROM rust:1.83-slim AS builder
RUN cargo install supergateway
RUN cargo install --git https://github.com/kneiht/leptos-mcp-server --locked

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/supergateway /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/leptos-mcp-server /usr/local/bin/
EXPOSE 8090
CMD ["supergateway", "--stdio", "leptos-mcp-server", "--port", "8090", "--host", "0.0.0.0"]
```

Pour les MCP en Node.js (plus courants) :

```dockerfile
FROM node:20-slim
RUN npm install -g supergateway @anthropic/mcp-server-exemple
EXPOSE 8090
CMD ["supergateway", "--stdio", "mcp-server-exemple", "--port", "8090", "--host", "0.0.0.0"]
```

### Phase 3 : Docker Compose

```yaml
name: mcp-servers

services:
  nginx:
    image: nginx:alpine
    restart: always
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx/mcp.conf:/etc/nginx/conf.d/default.conf
      - /etc/letsencrypt:/etc/letsencrypt:ro
    depends_on:
      - leptos
      # - axum
      # - tailwind

  leptos:
    build: ./servers/leptos
    restart: always
    expose:
      - "8090"

  # axum:
  #   build: ./servers/axum
  #   restart: always
  #   expose:
  #     - "8090"
```

### Phase 4 : Config Nginx

```nginx
server {
    listen 80;
    server_name mcp.DOMAINE.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name mcp.DOMAINE.com;

    ssl_certificate     /etc/letsencrypt/live/mcp.DOMAINE.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/mcp.DOMAINE.com/privkey.pem;

    # SSE : desactiver TOUT buffering
    proxy_buffering off;
    proxy_cache off;
    proxy_set_header Connection '';
    proxy_http_version 1.1;
    chunked_transfer_encoding off;
    proxy_read_timeout 86400s;    # SSE = connexions longues

    # Auth par API key (optionnel mais recommande)
    set $auth_ok 0;
    if ($http_authorization = "Bearer MON_SECRET_KEY") {
        set $auth_ok 1;
    }

    location /mcp/leptos/ {
        if ($auth_ok = 0) { return 403; }
        proxy_pass http://leptos:8090/;
    }

    # location /mcp/axum/ {
    #     if ($auth_ok = 0) { return 403; }
    #     proxy_pass http://axum:8090/;
    # }

    location /health {
        return 200 '{"status":"ok"}';
        add_header Content-Type application/json;
    }
}
```

### Phase 5 : Deployer sur le serveur

```bash
# 1. Cloner le repo sur le serveur
git clone git@github.com:TON_USER/mcp-servers.git
cd mcp-servers

# 2. Creer le certificat SSL (premiere fois)
sudo certbot certonly --standalone -d mcp.DOMAINE.com

# 3. Lancer
docker compose up -d --build

# 4. Verifier
curl https://mcp.DOMAINE.com/health
```

### Phase 6 : Configurer les clients

Dans n'importe quel projet, fichier `.mcp.json` :

```json
{
  "mcpServers": {
    "leptos": {
      "url": "https://mcp.DOMAINE.com/mcp/leptos/sse",
      "headers": {
        "Authorization": "Bearer MON_SECRET_KEY"
      }
    }
  }
}
```

Ou globalement dans `~/.claude/settings.json` pour tous les projets.

## Ajouter un nouveau MCP server

1. Creer `servers/nouveau-framework/Dockerfile`
2. Ajouter le service dans `docker-compose.yml`
3. Ajouter le `location` dans `nginx/mcp.conf`
4. `docker compose up -d --build nouveau-framework`
5. Tester : `curl https://mcp.DOMAINE.com/mcp/nouveau-framework/sse`

## MCP servers a considerer

| Framework | Source | Type |
|---|---|---|
| Leptos 0.8 | `github.com/kneiht/leptos-mcp-server` | Rust (cargo install) |
| Tailwind CSS | `@anthropic/mcp-server-tailwind` (si existe) ou custom | Node/Rust |
| sqlx | Custom (embedder la doc sqlx) | Rust |
| Axum | Custom (embedder la doc Axum) | Rust |
| Tokio | Custom | Rust |
| DDD/Hexagonal | Custom (patterns + recipes du projet) | Rust |

## Securite

- **HTTPS obligatoire** : Let's Encrypt gratuit
- **API key** : header `Authorization: Bearer` verifie par Nginx
- **Firewall** : seul le port 443 ouvert
- **Pas de secrets dans les MCP** : les serveurs ne servent que de la doc/analyse, pas d'acces DB
- **Rate limiting** (optionnel) : `limit_req_zone` dans Nginx

## Maintenance

```bash
# Mettre a jour un serveur (rebuild depuis GitHub)
docker compose build --no-cache leptos
docker compose up -d leptos

# Logs
docker compose logs -f leptos

# Mettre a jour tous les serveurs
docker compose build --no-cache
docker compose up -d
```

## Cout

- **Serveur** : un VPS a 5-10€/mois suffit (les MCP sont legers)
- **Domaine** : sous-domaine du domaine existant (gratuit)
- **SSL** : Let's Encrypt (gratuit)
- **Bande passante** : negligeable (que du texte JSON/SSE)
