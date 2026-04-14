# ai-core

Personal Docker infrastructure: self-hosted MCP servers.

## MCP Servers

**Self-hosted** — single multi-tenant endpoint `docs` at
`https://mcp.benoitpodwinski.com/mcp` (OAuth 2.1 PKCE). Filter content with
the `category` parameter on `search_docs` / `list_topics`:

| Category | Content |
|----------|---------|
| `leptos` | Leptos framework book |
| `leptos-use` | leptos-use hooks library |
| `rust` | The Rust Programming Language book |
| `daisyui` | DaisyUI component library |
| `induflow` | InduFlow PartHub API |
| `tailwindcss` | Tailwind CSS v4 — docs + full catalog |

The same endpoint also exposes `rust-docs-mcp` crate-analysis tools
(`cache_crate`, `search_crate_items`, `get_item_details`, …).

**External / stdio** (no self-hosted auth):

| Server | Type | Content |
|--------|------|---------|
| `openai-docs` | HTTP | OpenAI developer docs — API, Codex, ChatGPT Apps SDK |
| `ag-grid` | stdio (`npx`) | AG Grid documentation |
| `github` | stdio (binary) | GitHub — PRs, issues, repos (`export GITHUB_TOKEN=…`) |

### Claude Code

Drop `.mcp.json` at the project root:

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/claude-mcp.json > .mcp.json
```

Authentication is handled automatically via OAuth 2.1 PKCE.

### Codex CLI

Append the servers to `~/.codex/config.toml`:

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml \
     >> ~/.codex/config.toml
```

Then authenticate against the self-hosted endpoint (single OAuth flow):

```bash
codex mcp login docs

# GitHub (personal token)
export GITHUB_TOKEN=ghp_…
```

### Deploy

The [mcp-deploy.yml](.github/workflows/mcp-deploy.yml) workflow runs
automatically on push to `main` whenever files under `mcp/` change.

It uses a **self-hosted runner** installed on the server — no outbound SSH required.

**Runner install** (one-time, on the server):

```bash
# Repo Settings → Actions → Runners → New self-hosted runner → Linux
# Follow the instructions, then install as a service:
sudo ./svc.sh install && sudo ./svc.sh start

# The runner needs Docker access:
sudo usermod -aG docker <runner-user>
```

Required GitHub secret (`Settings → Secrets → Actions`):

| Secret | Example |
|--------|---------|
| `DEPLOY_PATH` | `/opt/mcp` |

### Updating configs

Configs are published automatically by GitHub Actions whenever
`mcp/servers-manifest.json` changes. Each release is versioned
(`mcp-configs-YYYYMMDD-<run>`) and tagged as latest — the
`/releases/latest/download/` URLs always point to the most recent version.

[See all releases →](https://github.com/bpodwinski/ai-core/releases)

```bash
node mcp/generate-configs.mjs   # regenerate locally (dist/ is gitignored)
```
