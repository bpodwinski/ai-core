# CLAUDE.md
<!-- sync: AGENTS.md est généré automatiquement depuis ce fichier -->

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Docker-based n8n workflow automation system with custom Python IMAP microservices, Qdrant vector database, and Cloudflare Tunnel for public HTTPS access. Includes a self-hosted MCP server infrastructure at `mcp/`.

## Common Commands

```bash
# Start all services
docker compose up -d

# Rebuild after code changes (especially Python scripts)
docker compose build --no-cache

# View logs
docker compose logs -f
docker compose logs -f python_script   # single service

# Restart a single service
docker compose restart python_script

# Test IMAP endpoints manually
curl http://localhost/count?mailbox=INBOX&criteria=ALL
curl "http://localhost/list?offset=0&limit=10&mailbox=INBOX"
```

No test framework, linter, or CI pipeline is configured.

## Architecture

Five Docker services orchestrated via `docker-compose.yml`:

- **n8n** (port 5678) — Workflow automation engine, stores workflows in PostgreSQL
- **python_script** (port 80) — FastAPI app (`scripts/api.py`) exposing IMAP operations as HTTP endpoints; calls `imap_count.py` and `imap_list_range_items.py` as subprocesses
- **qdrant** (ports 6333/6334) — Vector database for embeddings/semantic search
- **postgres** — PostgreSQL backing store for n8n (not exposed externally)
- **cf-tunnel** — Cloudflare Tunnel routing `n8n.benoitpodwinski.com` → n8n:5678 and `qdrant.benoitpodwinski.com` → qdrant:6333

## Key Files

- `docker-compose.yml` — Service definitions, networking, volumes
- `Dockerfile.python_script` — Python 3.14-slim image with FastAPI/Uvicorn
- `scripts/api.py` — FastAPI router: GET/POST `/count` and `/list` endpoints
- `scripts/imap_count.py` — Counts emails in a mailbox by criteria
- `scripts/imap_list_range_items.py` — Fetches paginated email list with optional body truncation
- `cloudflared/config.yml` — Cloudflare Tunnel ingress rules
- `.env_example` — Required environment variables (IMAP creds, encryption key, Postgres password, domain config)

## Environment Setup

Copy `.env_example` to `.env` and fill in all values before starting services. Key variables: `IMAP_HOST`, `IMAP_USER`, `IMAP_PASS`, `N8N_ENCRYPTION_KEY`, `POSTGRES_PASSWORD`, `DOMAIN_NAME`.

## Python Scripts

Scripts in `scripts/` run inside the `python_script` container. They use only standard library (`imaplib`, `email`, `argparse`, `json`) plus FastAPI/Uvicorn. Dependencies are installed in the Dockerfile — there is no `requirements.txt`.

The API layer (`api.py`) invokes scripts via `subprocess.run`, passing parameters as CLI arguments and returning stdout as JSON responses.

---

## Skills (Claude Code + Codex CLI)

Les skills réutilisables sont stockés en double dans deux répertoires, un par outil :

| Outil | Répertoire |
|-------|-----------|
| Claude Code | `.claude/skills/<name>/SKILL.md` |
| Codex CLI | `.agents/skills/<name>/SKILL.md` |

Le format `SKILL.md` est identique car les deux outils implémentent la norme ouverte [Agent Skills](https://agentskills.io). Les champs frontmatter spécifiques à Claude Code (`allowed-tools`, `argument-hint`, `context`, etc.) sont ignorés par Codex CLI.

Claude Code ne scanne pas `.agents/skills/` — les deux répertoires sont obligatoires.

**Convention** : quand tu crées ou modifies un skill, mets à jour les deux répertoires en parallèle. La source de vérité est `.claude/skills/` — recopie ensuite dans `.agents/skills/`.

### Skills disponibles

| Skill | Description |
|-------|-------------|
| `rust-unit-tests` | Analyse un projet Rust et génère les tests unitaires manquants |

---

## MCP Server Infrastructure

Self-hosted multi-tenant MCP docs server at `mcp/`, accessible via `https://mcp.benoitpodwinski.com/mcp`. A single Rust binary (`mcp-rust-docs`) loads all documentation from `/docs/` and serves them through one endpoint with category-based filtering.

### Architecture

```
Claude Code → HTTPS → Cloudflare Tunnel → nginx (OAuth auth_request) → mcp-docs:80
```

- Single multi-tenant container serving all doc sources
- OAuth 2.1 PKCE via `mcp/oauth/server.js`
- Transport: Streamable HTTP (`rmcp` Rust SDK)
- Client config: one entry `"docs"` in `.mcp.json`

### Doc sources (categories)

| Category | Source |
|----------|--------|
| `leptos` | `github.com/leptos-rs/book` |
| `leptos-use` | `github.com/synphonyte/leptos-use` |
| `rust` | `github.com/rust-lang/book` |
| `daisyui` | `https://daisyui.com/llms.txt` |
| `induflow` | Local docs in `mcp/servers/rust-docs/induflow/` |
| `tailwindcss` | `github.com/tailwindlabs/tailwindcss.com` + catalog généré |

### MCP Commands

```bash
cd mcp
just check        # Cargo check
just build        # Cargo build --release
just docker-build # Build Docker image
just deploy       # Deploy to server (rsync + docker compose)
just ship         # Build + deploy
just up           # Start locally
just down         # Stop
just logs         # View logs
```

### Adding a new doc source

When the user asks to add docs to the MCP server, follow these steps:

**1. Determine the doc source:**
- **GitHub repo with markdown docs** → add `git clone` in Dockerfile
- **Official `llms.txt` endpoint** → download it in Dockerfile and rename it to `.md`
- **Local docs** → place `.md` files in `mcp/servers/rust-docs/<name>/` and `COPY` in Dockerfile

**2. Update `mcp/servers/rust-docs/Dockerfile`:**
```dockerfile
# For GitHub source (stage node-builder):
RUN git clone --depth 1 https://github.com/<org>/<repo> /tmp/<name>
# In the runtime stage:
COPY --from=node-builder /tmp/<name>/<docs-path>/ /docs/<name>/

# For local docs:
COPY <name>/ /docs/<name>/
```

No changes needed to docker-compose.yml or nginx — the server auto-discovers all categories under `/docs/`.

**3. Update `mcp/servers-manifest.json`** — add entry in `"docSources"`.

**4. If the source has an OpenAPI spec (JSON/YAML):**
Convert it to markdown (`<name>-api-reference.md`) with endpoints, params, schemas, and auth details. Delete the original JSON/YAML/SVG files — the server only loads `.md`.

**5. Deploy:** `just ship`

**6. Verify:** `docker compose logs mcp-docs | grep "categories"` on the server.
