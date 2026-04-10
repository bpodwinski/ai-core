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

### Utiliser dans un projet

**Claude Code** — copier `.mcp.json` à la racine du projet :

```bash
curl -fsSL https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/claude-mcp.json > .mcp.json
```

**Codex CLI** — copier dans la config globale (nécessite `MCP_API_KEY`) :

```bash
curl -fsSL https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/codex-mcp.json > ~/.codex/mcp_servers.json
export MCP_API_KEY=<your-key>
```

### Mettre à jour les configs

Les fichiers `dist/` sont regénérés automatiquement par GitHub Actions dès que
`mcp/servers-manifest.json` change. Pour regénérer localement :

```bash
node mcp/generate-configs.mjs          # génère dist/
node mcp/generate-configs.mjs --check  # vérifie que dist/ est à jour
```
