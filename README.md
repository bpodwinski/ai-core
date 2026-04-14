# ai-core

Personal Docker infrastructure: self-hosted MCP servers.

## MCP Servers

MCP servers exposed via `https://mcp.benoitpodwinski.com/<name>/mcp`.

**Self-hosted** (`mcp.benoitpodwinski.com`):

| Server | Content |
|--------|---------|
| `leptos` | Leptos framework book |
| `leptos-use` | leptos-use hooks library |
| `rust` | The Rust Programming Language book |
| `daisyui` | DaisyUI component library |
| `induflow` | InduFlow PartHub API |
| `tailwindcss` | Tailwind CSS v4 — docs + full catalog |

**External** (public, no auth):

| Server | Content |
|--------|---------|
| `openai-docs` | OpenAI developer docs — API, Codex, ChatGPT Apps SDK |
| `github` | GitHub — PRs, issues, repos (`export GITHUB_TOKEN=…`) |

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

Then authenticate against the self-hosted servers:

```bash
# OAuth PKCE (per server)
codex mcp login leptos
codex mcp login leptos-use
codex mcp login rust
codex mcp login daisyui
codex mcp login induflow
codex mcp login tailwindcss

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
