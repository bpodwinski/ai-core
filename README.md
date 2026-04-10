# ai-core

Infrastructure Docker personnelle : n8n, Qdrant, serveurs MCP self-hosted.

## MCP Servers

Serveurs MCP accessibles via `https://mcp.benoitpodwinski.com/<name>/mcp`.

| Serveur | Contenu |
|---------|---------|
| `leptos` | Leptos framework book |
| `leptos-use` | leptos-use hooks library |
| `rust` | The Rust Programming Language book |
| `daisyui` | DaisyUI component library |
| `induflow` | InduFlow PartHub API |
| `tailwindcss` | Tailwind CSS v4 — docs + catalog complet |

### Claude Code

Copier `.mcp.json` à la racine du projet :

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/download/mcp-configs/claude-mcp.json > .mcp.json
```

L'authentification est gérée automatiquement via OAuth 2.1 PKCE.

### Codex CLI

Ajouter les serveurs à `~/.codex/config.toml` :

```bash
curl -fsSL https://github.com/bpodwinski/ai-core/releases/download/mcp-configs/codex-config.toml \
     >> ~/.codex/config.toml
```

Puis choisir une option d'authentification :

```bash
# Option A — Bearer token
export MCP_API_KEY=<your-key>

# Option B — OAuth PKCE (par serveur)
codex mcp login leptos
codex mcp login leptos-use
codex mcp login rust
codex mcp login daisyui
codex mcp login induflow
codex mcp login tailwindcss
```

### Mettre à jour les configs

Les configs sont publiées automatiquement par GitHub Actions comme assets de la
release [`mcp-configs`](https://github.com/bpodwinski/ai-core/releases/tag/mcp-configs)
dès que `mcp/servers-manifest.json` change. Pour regénérer localement :

```bash
node mcp/generate-configs.mjs   # génère dist/ (gitignored)
```
