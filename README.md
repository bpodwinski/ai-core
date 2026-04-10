# ai-core

Infrastructure Docker personnelle : n8n, Qdrant, serveurs MCP self-hosted.

## MCP Servers

Serveurs MCP accessibles via `https://mcp.benoitpodwinski.com/<name>/mcp`.

**Self-hosted** (`mcp.benoitpodwinski.com`) :

| Serveur | Contenu |
|---------|---------|
| `leptos` | Leptos framework book |
| `leptos-use` | leptos-use hooks library |
| `rust` | The Rust Programming Language book |
| `daisyui` | DaisyUI component library |
| `induflow` | InduFlow PartHub API |
| `tailwindcss` | Tailwind CSS v4 — docs + catalog complet |

**Externes** (publics, sans auth) :

| Serveur | Contenu |
|---------|---------|
| `openai-docs` | OpenAI developer docs — API, Codex, ChatGPT Apps SDK |
| `github` | GitHub — PRs, issues, repos (`export GITHUB_TOKEN=…`) |

### Claude Code

Copier `.mcp.json` à la racine du projet :

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/claude-mcp.json > .mcp.json
```

L'authentification est gérée automatiquement via OAuth 2.1 PKCE.

### Codex CLI

Ajouter les serveurs à `~/.codex/config.toml` :

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml \
     >> ~/.codex/config.toml
```

Puis s'authentifier sur les serveurs self-hosted :

```bash
# OAuth PKCE (par serveur)
codex mcp login leptos
codex mcp login leptos-use
codex mcp login rust
codex mcp login daisyui
codex mcp login induflow
codex mcp login tailwindcss

# GitHub (token personnel)
export GITHUB_TOKEN=ghp_…
```

### Mettre à jour les configs

Les configs sont publiées automatiquement par GitHub Actions à chaque changement de
`mcp/servers-manifest.json`. Chaque release est versionnée (`mcp-configs-YYYYMMDD-<run>`)
et marquée comme latest — les URLs `/releases/latest/download/` pointent toujours
sur la version la plus récente.

[Voir toutes les releases →](https://github.com/bpodwinski/ai-core/releases)

```bash
node mcp/generate-configs.mjs   # régénérer localement (dist/ gitignored)
```
