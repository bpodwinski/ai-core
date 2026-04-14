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
| `mcp-docs` | mcp-rust-docs:local | Serveur multi-tenant — docs + OAuth 2.1 PKCE |

Un seul container `mcp-docs` sert toutes les sources de documentation **et** les endpoints OAuth (discovery, authorize, token, validate). Les docs sont chargées une seule fois au démarrage depuis `/docs/` et partagées entre toutes les sessions via `Arc`. Chaque sous-dossier de `/docs/` devient une catégorie (`leptos`, `rust`, `daisyui`, etc.).

## Fichiers clés

| Fichier | Rôle |
|---------|------|
| `justfile` | Tâches build/deploy (Just task runner) |
| `docker-compose.yml` | Orchestration des 3 services |
| `servers-manifest.json` | Source de vérité : doc sources, types, transforms, configs clients |
| `generate-configs.mjs` | Génère `dist/claude-mcp.json` et `dist/codex-config.toml` |
| `deploy.ps1` | Déploiement SSH → serveur distant |
| `.dockerignore` | Exclut target/, node_modules/, configs du build context |
| `nginx/mcp.conf.template` | Config nginx : auth, rate-limit, proxy vers mcp-docs |
| `cloudflared/config.yml` | Tunnel Cloudflare → nginx:80 |
| `servers/rust-docs/src/main.rs` | Entry point Rust — chargement docs, OAuth, health check enrichi |
| `servers/rust-docs/src/oauth.rs` | OAuth 2.1 PKCE (RFC 8414/7591/9728), axum |
| `servers/rust-docs/src/views/authorize.html` | Page de consentement (chargée via `include_str!`) |
| `servers/rust-docs/src/tools.rs` | 3 outils MCP : search_docs, get_doc, list_topics |
| `servers/rust-docs/Dockerfile` | Build multi-stage : Rust + docs (config-driven via manifest) |
| `servers/rust-docs/fetch-docs.mjs` | Lit le manifest, fetch & transforme toutes les docs |
| `servers/rust-docs/local-docs/` | Docs locales (induflow/, etc.) |

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

## Serveur OAuth (servers/rust-docs/src/oauth.rs)

OAuth 2.1 PKCE minimaliste, intégré au binaire `mcp-rust-docs` (axum). Tokens d'1h, Bearer = `MCP_API_KEY`.  
Pas de BDD — codes d'autorisation en mémoire. Redémarrer = invalider les flows en cours.

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

## Gestion de la config client (.mcp.json)

**Ne jamais éditer `.mcp.json` directement.** Il est généré depuis `servers-manifest.json`.

```bash
node generate-configs.mjs        # génère dist/claude-mcp.json + codex-config.toml
cp dist/claude-mcp.json ../.mcp.json
```

### Ajouter un serveur MCP HTTP externe

Ajouter dans `servers-manifest.json` → `"external"` :

```jsonc
{
  "name": "<name>",
  "url": "https://example.com/mcp",
  "description": "...",
  "bearer_token_env_var": "MY_TOKEN"   // optionnel
}
```

### Ajouter un serveur MCP stdio (npx / process local)

Ajouter dans `servers-manifest.json` → `"stdio"` :

```jsonc
{
  "name": "<name>",
  "command": "npx",
  "args": ["<package-name>"],
  "description": "..."
}
```

Puis régénérer : `node generate-configs.mjs && cp dist/claude-mcp.json ../.mcp.json`

---

## Outils d'analyse de crates Rust (rust-docs-mcp)

Le serveur `mcp-docs` embarque `rust-docs-mcp` (snowmead) comme subprocess persistant. Il génère le rustdoc JSON à la demande et expose des outils d'analyse de crates sur le même endpoint `/mcp`.

**Mettre une crate en cache** — dire à Claude :
> "Cache la crate axum version 0.8.8"

**Outils disponibles :**

| Tool | Description |
|------|-------------|
| `cache_crate` | Télécharge et cache une crate (cratesio / github / local). Params: `members` (workspaces), `update` (force re-cache), `features` (features spécifiques pour compat mutually exclusive) |
| `list_cached_crates` | Liste les crates en cache avec leur taille |
| `list_crate_versions` | Liste les versions disponibles sur crates.io |
| `search_crate_items` | Recherche par pattern (`crate_name`, `version`, `pattern`). Params optionnels : `kind_filter`, `limit`, `offset`, `path_filter`, `member` |
| `search_crate_items_fuzzy` | Recherche floue tolérante aux typos (`query`, `fuzzy_distance` 0-2) |
| `list_crate_items` | Liste tous les items d'une crate (avec pagination et `kind_filter`) |
| `get_item_details` | Signature complète + docs d'un item (`item_id` entier i32) |
| `get_item_docs` | Documentation seule d'un item (plus léger que `get_item_details`) |
| `get_item_source` | Code source d'un item |
| `get_crate_dependencies` | Arbre de dépendances |
| `remove_crate` | Supprime une crate du cache |

**Cache** persistant dans le volume Docker `rust_docs_cache`.

**Prérequis runtime :** toolchain nightly dans le container (`RUST_DOCS_MCP_TOOLCHAIN=nightly`), packages `build-essential pkg-config libssl-dev libclang-dev` (requis pour crates comme sqlx/bindgen).

**Variables d'environnement runtime :**
- `RUSTFLAGS="--cfg tokio_unstable --cfg reqwest_unstable"` — active les features unstable de tokio et reqwest
- `RUSTDOCFLAGS="--cfg tokio_unstable --cfg reqwest_unstable"` — idem pour rustdoc

**Paramètre `features` :** rust-docs-mcp utilise `--all-features` par défaut. Pour les crates avec features mutuellement exclusives, utiliser le paramètre `features` :

```
cache_crate({ crate_name: "leptos-use", source_type: "cratesio", version: "0.15.8", features: ["axum"] })
cache_crate({ crate_name: "chrono", source_type: "cratesio", version: "0.4.41", features: [] })
```

Sans `features`, la stratégie `--all-features` par défaut est utilisée avec fallback automatique.

---

## Ajouter une nouvelle source de docs (config-driven)

Le build est piloté par `servers-manifest.json`. Ajouter une doc source = **modifier uniquement le manifest** (pas de Dockerfile/docker-compose/nginx à toucher).

**1. Ajouter l'entrée dans `servers-manifest.json` → `"docSources"` :**

```jsonc
// Repo GitHub :
{ "name": "<name>", "description": "...", "source": { "type": "git", "url": "https://github.com/<org>/<repo>", "docsPath": "src/" } }

// URL unique (llms.txt) :
{ "name": "<name>", "description": "...", "source": { "type": "url", "url": "https://example.com/llms.txt", "transforms": ["split"] } }

// Docs locales :
{ "name": "<name>", "description": "...", "source": { "type": "local", "path": "<name>/" } }
```

Pour les docs locales, placer les `.md` dans `servers/rust-docs/local-docs/<name>/`.

**2. Transforms disponibles** (optionnels, appliqués dans l'ordre) :
- `strip-mdx` — convertit `.mdx` → `.md` via remark AST
- `generate-catalog` — génère le catalog des classes Tailwind CSS
- `split` — découpe un fichier unique en un `.md` par heading

**3. Si la source est une spec OpenAPI (JSON/YAML) :** convertir en Markdown, supprimer les fichiers JSON/YAML/SVG — le serveur ne charge que les `.md`.

**4. Déployer et vérifier :**
```bash
just ship
# Sur le serveur distant :
docker compose logs mcp-docs | grep "categories"
```

Le `fetch-docs.mjs` lit le manifest et orchestre le fetch/transform automatiquement. Le serveur détecte les nouveaux sous-dossiers dans `/docs/` comme catégories.

## Génération des configs clients

```bash
node generate-configs.mjs
# Produit : dist/claude-mcp.json et dist/codex-config.toml
```

Ces fichiers sont publiés comme assets de release GitHub.
