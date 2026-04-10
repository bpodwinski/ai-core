# CLAUDE.md — mcp/

Infrastructure MCP auto-hébergée exposant des serveurs de documentation via HTTPS.  
URL publique : `https://mcp.benoitpodwinski.com/<name>/mcp`

## Commandes courantes

```bash
# Depuis e:/Dev/ai-core/mcp/
just check          # Cargo check (sans compiler)
just build          # Compile le binaire Rust en release
just docker-build   # Build l'image Docker (mcp-rust-docs:local)
just up             # Démarre toute la stack localement
just down           # Arrête la stack
just logs           # Tous les logs en continu
just logs mcp-rust  # Logs d'un seul service
just restart mcp-leptos  # Redémarre un service
just deploy         # Déploie sur le serveur distant (rsync + docker compose)
just ship           # docker-build + deploy
```

## Architecture

```
Claude Code → HTTPS → Cloudflare Tunnel → nginx (auth_request OAuth) → mcp-<name>:80
```

**Services Docker** (`docker-compose.yml`) :

| Service | Image | Rôle |
|---------|-------|------|
| `cf-tunnel` | cloudflare/cloudflared | Expose nginx publiquement |
| `nginx` | nginx:alpine | Reverse proxy + rate limiting + auth |
| `oauth` | build: ./oauth/ | Serveur OAuth 2.1 PKCE (Node.js) |
| `mcp-leptos` | mcp-rust-docs:local | Docs Leptos framework |
| `mcp-leptos-use` | mcp-rust-docs:local | Docs leptos-use |
| `mcp-rust` | mcp-rust-docs:local | Docs The Rust Book |
| `mcp-daisyui` | mcp-rust-docs:local | Docs DaisyUI |
| `mcp-induflow` | mcp-rust-docs:local | Docs InduFlow API |
| `mcp-tailwindcss` | mcp-rust-docs:local | Docs Tailwind CSS v4 |

Tous les services MCP partagent le **même binaire Rust** (`mcp-rust-docs`) — seul `DOCS_PATH` change.

## Fichiers clés

| Fichier | Rôle |
|---------|------|
| `justfile` | Tâches build/deploy (Just task runner) |
| `docker-compose.yml` | Orchestration des 8 services |
| `servers-manifest.json` | Métadonnées des serveurs (utilisé par generate-configs.mjs) |
| `generate-configs.mjs` | Génère `dist/claude-mcp.json` et `dist/codex-config.toml` |
| `deploy.ps1` | Déploiement SSH → serveur distant |
| `nginx/mcp.conf.template` | Config nginx : auth, rate-limit, proxy vers chaque service |
| `cloudflared/config.yml` | Tunnel Cloudflare → nginx:80 |
| `oauth/server.js` | OAuth 2.0 PKCE (RFC 8414/7591/9728), Express.js |
| `servers/rust-docs/src/main.rs` | Entry point Rust — StreamableHttpService, route /health |
| `servers/rust-docs/src/tools.rs` | 3 outils MCP : search_docs, get_doc, list_topics |
| `servers/rust-docs/Dockerfile` | Build multi-stage : Rust + Node + docs |
| `servers/rust-docs/induflow/` | Docs InduFlow locales (fichiers .md) |

## Serveur Rust (servers/rust-docs/)

Binaire unique compilé en Rust avec `rmcp`. Lit les `.md` depuis `DOCS_PATH` au démarrage.

**Outils MCP exposés :**
- `list_topics` — liste les docs disponibles (filtre par catégorie optionnel)
- `search_docs` — recherche full-text, retourne 20 résultats max
- `get_doc` — récupère un doc par chemin `catégorie/topic`

**Endpoints HTTP :**
- `POST /mcp` — transport Streamable HTTP (SSE)
- `GET /health` — healthcheck (utilisé par Docker)

## Serveur OAuth (oauth/server.js)

OAuth 2.1 PKCE minimaliste. Tokens longue durée (1 an) depuis `MCP_API_KEY`.  
Pas de BDD — tout en mémoire. Redémarrer = invalider les sessions en cours.

**Endpoints :**
- `GET /.well-known/oauth-authorization-server` — métadonnées RFC 8414
- `POST /oauth/register` — enregistrement dynamique (RFC 7591)
- `GET /oauth/authorize` + `GET /oauth/approve` — flow PKCE
- `POST /oauth/token` — échange code → token
- `GET /oauth/validate` — validation Bearer pour nginx `auth_request`

## Variables d'environnement

Copier `.env_example` → `.env` :

```
MCP_TUNNEL_TOKEN=   # Token du tunnel Cloudflare
OAUTH_ISSUER=       # https://mcp.benoitpodwinski.com
MCP_API_KEY=        # Clé API longue (utilisée comme Bearer token)
```

Copier `.env.deploy.example` → `.env.deploy` pour le déploiement SSH :

```
REMOTE_USER=
REMOTE_HOST=
REMOTE_PORT=22
REMOTE_PATH=/public_html
SSH_KEY=~/.ssh/mcp_deploy
```

## Ajouter un nouveau serveur MCP

**1. Source des docs :**
- GitHub → `git clone --depth 1` dans le Dockerfile (stage node-builder)
- `llms.txt` officiel → `curl -L <url> -o /tmp/<name>-docs.md`
- Docs locales → placer les `.md` dans `servers/rust-docs/<name>/`

**2. `servers/rust-docs/Dockerfile`** — ajouter dans le stage node-builder et copier dans le runtime :
```dockerfile
# Stage node-builder
RUN git clone --depth 1 https://github.com/<org>/<repo> /tmp/<name>
# Stage runtime
COPY --from=node-builder /tmp/<name>/<docs-path>/ /docs/<name>/
```

**3. `docker-compose.yml`** — ajouter le service :
```yaml
mcp-<name>:
  image: mcp-rust-docs:local
  container_name: mcp-<name>
  restart: unless-stopped
  expose:
    - "80"
  environment:
    DOCS_PATH: /docs/<name>
  healthcheck:
    test: ["CMD", "wget", "-qO-", "http://127.0.0.1/health"]
    interval: 30s
    timeout: 5s
    retries: 3
    start_period: 15s
```
Ajouter `mcp-<name>: {condition: service_healthy}` dans `depends_on` de nginx.

**4. `nginx/mcp.conf.template`** — ajouter le bloc location :
```nginx
location /<name>/ {
    auth_request            /auth-check;
    proxy_pass              http://mcp-<name>:80/;
    proxy_buffering         off;
    proxy_cache             off;
    proxy_set_header        Connection '';
    proxy_set_header        Host $host;
    proxy_http_version      1.1;
    chunked_transfer_encoding off;
    proxy_read_timeout      86400s;
}
```

**5. `servers-manifest.json`** — ajouter l'entrée dans `"servers"`.

**6. Si la source est une spec OpenAPI (JSON/YAML) :** convertir en Markdown, supprimer les fichiers JSON/YAML/SVG — le serveur ne charge que les `.md`.

**7. Déployer et vérifier :**
```bash
just ship
# Sur le serveur distant :
docker compose logs mcp-<name> | grep "Loaded"
```

## Génération des configs clients

```bash
node generate-configs.mjs
# Produit : dist/claude-mcp.json et dist/codex-config.toml
```

Ces fichiers sont publiés comme assets de release GitHub.
