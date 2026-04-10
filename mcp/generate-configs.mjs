#!/usr/bin/env node
/**
 * Génère les fichiers de configuration MCP pour Claude Code et Codex CLI
 * à partir de mcp/servers-manifest.json.
 *
 * Usage :
 *   node mcp/generate-configs.mjs   # écrit dans dist/
 *
 * Les fichiers générés sont publiés automatiquement via GitHub Actions
 * comme assets de la release "mcp-configs".
 */

import { readFileSync, writeFileSync, mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// ── Source de vérité ─────────────────────────────────────────────────────────
const manifest = JSON.parse(
  readFileSync(resolve(__dirname, 'servers-manifest.json'), 'utf8')
);
const { baseUrl, servers } = manifest;

// ── Claude Code : claude-mcp.json → .mcp.json ────────────────────────────────
// OAuth 2.1 PKCE géré nativement par Claude Code (pas d'auth explicite).
// Copier à la racine du projet : curl … > .mcp.json
const claudeConfig = {
  mcpServers: Object.fromEntries(
    servers.map(s => [
      s.name,
      { type: 'http', url: `${baseUrl}/${s.name}/mcp` }
    ])
  )
};

// ── Codex CLI : codex-config.toml → ~/.codex/config.toml ─────────────────────
// Format TOML avec [mcp_servers.<name>] tables.
// Deux options d'auth :
//   - bearer_token_env_var : token statique dans MCP_API_KEY
//   - OAuth PKCE           : après ajout, lancer `codex mcp login <name>`
function generateCodexToml() {
  const header = [
    '# Codex CLI — MCP servers configuration',
    '# Source : https://github.com/bpodwinski/ai-core/releases/download/mcp-configs/codex-config.toml',
    '#',
    '# Installation :',
    '#   curl -fsSL https://github.com/bpodwinski/ai-core/releases/download/mcp-configs/codex-config.toml \\',
    '#        >> ~/.codex/config.toml',
    '#',
    '# Auth (choisir une option) :',
    '#   Option A — Bearer token : export MCP_API_KEY=<your-key>',
    '#   Option B — OAuth PKCE   : codex mcp login <server-name>',
    '',
  ].join('\n');

  const blocks = servers.map(s => [
    `[mcp_servers.${s.name}]`,
    `url = "${baseUrl}/${s.name}/mcp"`,
    `bearer_token_env_var = "MCP_API_KEY"`,
    `# description = "${s.description}"`,
    '',
  ].join('\n'));

  return header + blocks.join('\n');
}

// ── Écriture ──────────────────────────────────────────────────────────────────
const distDir = resolve(__dirname, '../dist');
mkdirSync(distDir, { recursive: true });

const outputs = [
  { path: resolve(distDir, 'claude-mcp.json'), content: JSON.stringify(claudeConfig, null, 2) + '\n' },
  { path: resolve(distDir, 'codex-config.toml'), content: generateCodexToml() },
];

for (const { path, content } of outputs) {
  writeFileSync(path, content);
  console.log(`Written: ${path}`);
}

console.log(`\n${servers.length} servers — configs générées dans dist/`);
