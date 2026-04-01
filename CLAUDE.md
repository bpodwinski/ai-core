# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Docker-based n8n workflow automation system with custom Python IMAP microservices, Qdrant vector database, and Cloudflare Tunnel for public HTTPS access.

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
