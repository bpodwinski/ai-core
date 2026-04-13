# CLAUDE.md — mcp/
<!-- sync: AGENTS.md est généré automatiquement depuis ce fichier -->

Infrastructure MCP auto-hébergée exposant un serveur de documentation multi-tenant via HTTPS.  
URL publique : `https://mcp.benoitpodwinski.com/mcp`

## Commandes courantes

```bash
# Depuis e:/Dev/ai-core/mcp/
just check          # Cargo check (sans compiler)
just build          # Compile le binaire Rust en release
just docker-build   # Build l'image Docker (mcp-rust-docs:local)
just up             # Démarre toute la stack localement
just down           # Arrête la stack
just logs           # Tous les logs en continu
just logs mcp-docs  # Logs du serveur docs
just restart mcp-docs  # Redémarre le serveur docs
just deploy         # Déploie sur le serveur distant (rsync + docker compose)
just ship           # docker-build + deploy
```

## Architecture

```
Claude Code → HTTPS → Cloudflare Tunnel → nginx (auth_request OAuth) → mcp-docs:80
```

**Services Docker** (`docker-compose.yml`) :

| Service | Image | Rôle |
|---------|-------|------|
| `cf-tunnel` | cloudflare/cloudflared | Expose nginx publiquement |
| `nginx` | nginx:alpine | Reverse proxy + rate limiting + auth |
| `oauth` | build: ./oauth/ | Serveur OAuth 2.1 PKCE (Node.js) |
| `mcp-docs` | mcp-rust-docs:local | Serveur multi-tenant — toutes les docs |

Un seul container `mcp-docs` sert toutes les sources de documentation. Les docs sont chargées une seule fois au démarrage depuis `/docs/` et partagées entre toutes les sessions via `Arc`. Chaque sous-dossier de `/docs/` devient une catégorie (`leptos`, `rust`, `daisyui`, etc.).

## Fichiers clés

| Fichier | Rôle |
|---------|------|
| `justfile` | Tâches build/deploy (Just task runner) |
| `docker-compose.yml` | Orchestration des 4 services |
| `servers-manifest.json` | Métadonnées des doc sources (utilisé par generate-configs.mjs) |
| `generate-configs.mjs` | Génère `dist/claude-mcp.json` et `dist/codex-config.toml` |
| `deploy.ps1` | Déploiement SSH → serveur distant |
| `nginx/mcp.conf.template` | Config nginx : auth, rate-limit, proxy vers mcp-docs |
| `cloudflared/config.yml` | Tunnel Cloudflare → nginx:80 |
| `oauth/server.js` | OAuth 2.0 PKCE (RFC 8414/7591/9728), Express.js |
| `servers/rust-docs/src/main.rs` | Entry point Rust — chargement docs, health check enrichi |
| `servers/rust-docs/src/tools.rs` | 3 outils MCP : search_docs, get_doc, list_topics |
| `servers/rust-docs/Dockerfile` | Build multi-stage : Rust + Node + docs |
| `servers/rust-docs/induflow/` | Docs InduFlow locales (fichiers .md) |

## Serveur Rust (servers/rust-docs/)

Binaire unique compilé en Rust avec `rmcp`. Charge toutes les docs depuis `DOCS_PATH` une seule fois au démarrage, partage en mémoire via `Arc` entre les sessions.

**Outils MCP exposés :**
- `list_topics(category?)` — liste les docs disponibles (filtre par catégorie optionnel)
- `search_docs(query, category?)` — recherche full-text avec filtrage par catégorie, retourne 20 résultats max
- `get_doc(path)` — récupère un doc par chemin `catégorie/topic`

**Validation des inputs :**
- `search_docs` : query limitée à 500 caractères
- `get_doc` : protection contre path traversal (`..`, `/` initial)

**Endpoints HTTP :**
- `POST /mcp` — transport Streamable HTTP (SSE)
- `GET /health` — healthcheck enrichi (status, docs_loaded, categories)

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

## Ajouter une nouvelle source de docs

Grâce à l'architecture multi-tenant, ajouter des docs ne nécessite plus de toucher au docker-compose ni à nginx.

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

**3. `servers-manifest.json`** — ajouter l'entrée dans `"docSources"`.

**4. Si la source est une spec OpenAPI (JSON/YAML) :** convertir en Markdown, supprimer les fichiers JSON/YAML/SVG — le serveur ne charge que les `.md`.

**5. Déployer et vérifier :**
```bash
just ship
# Sur le serveur distant :
docker compose logs mcp-docs | grep "categories"
```

Le serveur détecte automatiquement le nouveau sous-dossier dans `/docs/` comme catégorie.

## Génération des configs clients

```bash
node generate-configs.mjs
# Produit : dist/claude-mcp.json et dist/codex-config.toml
```

Ces fichiers sont publiés comme assets de release GitHub.
